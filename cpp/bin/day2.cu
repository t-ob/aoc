// max games = 6

#include <string>
#include <iostream>
#include <sstream>

static const int MAX_GAMES = 100;
static const int MAX_ROUNDS = 6;
static const int CHANNELS = 3;
static const int BLOCKS = 4;
static const int GAME_THREADS_PER_BLOCK = 32;
static const int ROUND_THREADS_PER_BLOCK = 8;
static const int RGB_THREADS_PER_BLOCK = 4;

__global__ void
kernel(const uint8_t *games, const uint8_t *rounds, const uint8_t *rgb, uint32_t *outPart1, uint32_t *outPart2) {
    __shared__ bool part1Data[GAME_THREADS_PER_BLOCK][ROUND_THREADS_PER_BLOCK][RGB_THREADS_PER_BLOCK];
    __shared__ uint32_t part2Data[GAME_THREADS_PER_BLOCK][ROUND_THREADS_PER_BLOCK][RGB_THREADS_PER_BLOCK];
    __shared__ bool part1RoundIsSolvable[GAME_THREADS_PER_BLOCK][ROUND_THREADS_PER_BLOCK];
    __shared__ uint32_t part1GameIdSum[GAME_THREADS_PER_BLOCK];
    __shared__ uint32_t part2PowerSum[GAME_THREADS_PER_BLOCK];

    // Will answer "does the channel value at a given round for a given game satisfy the target"
    part1Data[threadIdx.x][threadIdx.y][threadIdx.z] = false;

    __syncthreads();

    // Don't do work on threads where none is required
    auto gameIdx = blockIdx.x * blockDim.x + threadIdx.x;
    if (gameIdx >= MAX_GAMES) return;
    if (threadIdx.y >= rounds[gameIdx]) return;
    if (threadIdx.z >= CHANNELS) return;

    auto target = rgb[threadIdx.z];
    // Do the uint8_t[100][6][3] to *uint8_t index arithmetic
    auto index = gameIdx * (MAX_ROUNDS * CHANNELS) + threadIdx.y * CHANNELS + threadIdx.z;
    auto value = games[index];

    part1Data[threadIdx.x][threadIdx.y][threadIdx.z] = value <= target;

    // We'll reduce this up the y-axis later for the power components of part 2
    part2Data[threadIdx.x][threadIdx.y][threadIdx.z] = value;

    __syncthreads();

    // Determine which games are solvable for part 1
    if (threadIdx.z == 0) {
        auto outcome = part1Data[threadIdx.x][threadIdx.y];
        part1RoundIsSolvable[threadIdx.x][threadIdx.y] = outcome[0] && outcome[1] && outcome[2];

        __syncthreads();

        for (auto s = ROUND_THREADS_PER_BLOCK >> 1; s > 0; s >>= 1) {
            if (threadIdx.y < s && threadIdx.y + s < rounds[gameIdx]) {
                part1RoundIsSolvable[threadIdx.x][threadIdx.y] = part1RoundIsSolvable[threadIdx.x][threadIdx.y] &&
                                                                 part1RoundIsSolvable[threadIdx.x][threadIdx.y + s];
            }
            __syncthreads();
        }
    }

    // Add the IDs of the solvable games
    if (threadIdx.y == 0 && threadIdx.z == 0) {
        part1GameIdSum[threadIdx.x] = static_cast<uint32_t>(part1RoundIsSolvable[threadIdx.x][0]) * (gameIdx + 1);

        __syncthreads();

        for (auto s = GAME_THREADS_PER_BLOCK >> 1; s > 0; s >>= 1) {
            if (threadIdx.x < s) {
                part1GameIdSum[threadIdx.x] += part1GameIdSum[threadIdx.x + s];
            }
            __syncthreads();
        }
    }

    // Move the final output at position 0 to the correct part of the output
    if (threadIdx.x == 0 && threadIdx.y == 0 && threadIdx.z == 0) outPart1[blockIdx.x] = part1GameIdSum[0];

    __syncthreads();

    // Build up the max across each channel for all rounds in a game
    for (auto s = ROUND_THREADS_PER_BLOCK >> 1; s > 0; s >>= 1) {
        if (threadIdx.y < s) {
            part2Data[threadIdx.x][threadIdx.y][threadIdx.z] = max(part2Data[threadIdx.x][threadIdx.y][threadIdx.z],
                                                                   part2Data[threadIdx.x][threadIdx.y +
                                                                                          s][threadIdx.z]);
        }
        __syncthreads();
    }

    // Compute their power and sum them
    if (threadIdx.y == 0 && threadIdx.z == 0) {
        part2PowerSum[threadIdx.x] = (part2Data[threadIdx.x][0][0] * part2Data[threadIdx.x][0][1] *
                                      part2Data[threadIdx.x][0][2]);
        __syncthreads();

        for (auto s = GAME_THREADS_PER_BLOCK >> 1; s > 0; s >>= 1) {
            if (threadIdx.x < s) {
                part2PowerSum[threadIdx.x] += part2PowerSum[threadIdx.x + s];
            }
            __syncthreads();
        }
    }

    // Move the final output at position 0 to the correct part of the output
    if (threadIdx.x == 0 && threadIdx.y == 0 && threadIdx.z == 0) outPart2[blockIdx.x] = part2PowerSum[0];
}

int main() {
    uint8_t games[MAX_GAMES][MAX_ROUNDS][CHANNELS] = {};
    uint8_t rounds[MAX_GAMES] = {};

    // Parse input data
    auto game = 0;
    std::string line;
    while (std::getline(std::cin, line)) {
        size_t pos = line.find(':');
        line = line.substr(pos + 1);

        std::istringstream ss(line);
        std::string record;
        int round = 0;

        while (std::getline(ss, record, ';') && round < MAX_ROUNDS) {
            std::istringstream recordStream(record);
            std::string word;
            int count;

            for (int channel = 0; channel < CHANNELS; ++channel) {
                games[game][round][channel] = 0;
            }

            while (recordStream >> count >> word) {
                if (word.ends_with(',')) word.pop_back();
                if (word == "red" || word == "red,") {
                    games[game][round][0] = count;
                } else if (word == "green") {
                    games[game][round][1] = count;
                } else if (word == "blue") {
                    games[game][round][2] = count;
                }
            }

            round++;
        }

        rounds[game] = round;
        ++game;
    }

    // Move parsed input to device
    uint8_t *dGames;
    cudaMalloc(&dGames, sizeof(games));
    cudaMemcpy(dGames, games, sizeof(games), cudaMemcpyHostToDevice);

    // Move rounds per game data to device
    uint8_t *dRounds;
    cudaMalloc(&dRounds, sizeof(rounds));
    cudaMemcpy(dRounds, rounds, sizeof(rounds), cudaMemcpyHostToDevice);

    // Move part 1 targets to device
    uint8_t rgb[3] = {12, 13, 14};
    uint8_t *dRgb;
    cudaMalloc(&dRgb, sizeof(rgb));
    cudaMemcpy(dRgb, rgb, sizeof(rgb), cudaMemcpyHostToDevice);

    // Allocate memory for output for both parts
    uint32_t *dOutPart1;
    cudaMalloc(&dOutPart1, sizeof(uint32_t) * BLOCKS);
    uint32_t *dOutPart2;
    cudaMalloc(&dOutPart2, sizeof(uint32_t) * BLOCKS);

    dim3 threadsPerBlock(GAME_THREADS_PER_BLOCK, ROUND_THREADS_PER_BLOCK, RGB_THREADS_PER_BLOCK);

    kernel<<<BLOCKS, threadsPerBlock>>>(dGames, dRounds, dRgb, dOutPart1, dOutPart2);
    cudaDeviceSynchronize();

    // Retrieve outputs
    uint32_t outPart1[BLOCKS];
    cudaMemcpy(outPart1, dOutPart1, sizeof(uint32_t) * BLOCKS, cudaMemcpyDeviceToHost);
    uint32_t outPart2[BLOCKS];
    cudaMemcpy(outPart2, dOutPart2, sizeof(uint32_t) * BLOCKS, cudaMemcpyDeviceToHost);

    uint32_t totalPart1 = 0;
    for (const auto s: outPart1) totalPart1 += s;

    uint32_t totalPart2 = 0;
    for (const auto s: outPart2) totalPart2 += s;

    std::cout << totalPart1 << std::endl;
    std::cout << totalPart2 << std::endl;

    return 0;
}