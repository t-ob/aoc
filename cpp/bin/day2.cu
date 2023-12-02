// max games = 6

#include <string>
#include <iostream>
#include <sstream>

static const int BLOCKS = 4;
static const int GAME_THREADS_PER_BLOCK = 32;
static const int ROUND_THREADS_PER_BLOCK = 8;
static const int RGB_THREADS_PER_BLOCK = 4;

__global__ void kernel(const uint8_t* games, const uint8_t* rounds, const uint8_t *rgb, uint32_t *outPart1, uint32_t *outPart2) {
    __shared__ bool part1Data[32][8][4];
    __shared__ uint32_t part2Data[32][8][4];
    __shared__ bool part1RoundIsSolvable[32][8];
    __shared__ uint32_t part1GameIdSum[32];
    __shared__ uint32_t part2PowerSum[32];

    part1Data[threadIdx.x][threadIdx.y][threadIdx.z] = false;

    __syncthreads();

    auto gameIdx = blockIdx.x * blockDim.x + threadIdx.x;
    if (gameIdx >= 100) return;

    if (threadIdx.y >= rounds[gameIdx]) return;

    if (threadIdx.z >= 3) return;

    auto target = rgb[threadIdx.z];
    auto index = gameIdx * (6 * 3) + threadIdx.y * 3 + threadIdx.z;
    auto value = games[index];

    part1Data[threadIdx.x][threadIdx.y][threadIdx.z] = value <= target;
    part2Data[threadIdx.x][threadIdx.y][threadIdx.z] = value;

    __syncthreads();

    if (threadIdx.z == 0) {
        auto outcome = part1Data[threadIdx.x][threadIdx.y];
        part1RoundIsSolvable[threadIdx.x][threadIdx.y] = outcome[0] && outcome[1] && outcome[2];

        __syncthreads();

        for (auto s = 8 >> 1; s > 0; s >>= 1) {
            if (threadIdx.y < s && threadIdx.y + s < rounds[gameIdx]) {
                part1RoundIsSolvable[threadIdx.x][threadIdx.y] = part1RoundIsSolvable[threadIdx.x][threadIdx.y] && part1RoundIsSolvable[threadIdx.x][threadIdx.y + s];
            }
            __syncthreads();
        }
    }

    if (threadIdx.y == 0 && threadIdx.z == 0) {
        part1GameIdSum[threadIdx.x] = static_cast<uint32_t>(part1RoundIsSolvable[threadIdx.x][0]) * (gameIdx + 1);

        __syncthreads();

        for (auto s = 32 >> 1; s > 0; s >>= 1) {
            if (threadIdx.x < s) {
                part1GameIdSum[threadIdx.x] += part1GameIdSum[threadIdx.x + s];
            }
            __syncthreads();
        }
    }

    if (threadIdx.x == 0 && threadIdx.y == 0 && threadIdx.z == 0) outPart1[blockIdx.x] = part1GameIdSum[0];

    __syncthreads();

    for (auto s = 8 >> 1; s > 0; s >>= 1) {
        if (threadIdx.y < s) {
            part2Data[threadIdx.x][threadIdx.y][threadIdx.z] = max(part2Data[threadIdx.x][threadIdx.y][threadIdx.z], part2Data[threadIdx.x][threadIdx.y + s][threadIdx.z]);
        }
        __syncthreads();
    }

    if (threadIdx.y == 0 && threadIdx.z == 0) {
        part2PowerSum[threadIdx.x] = (part2Data[threadIdx.x][0][0] * part2Data[threadIdx.x][0][1] * part2Data[threadIdx.x][0][2]);
        __syncthreads();

        for (auto s = 32 >> 1; s > 0; s >>= 1) {
            if (threadIdx.x < s) {
                part2PowerSum[threadIdx.x] += part2PowerSum[threadIdx.x + s];
            }
            __syncthreads();
        }
    }
    if (threadIdx.x == 0 && threadIdx.y == 0 && threadIdx.z == 0) outPart2[blockIdx.x] = part2PowerSum[0];
}

int main() {
    uint8_t games[100][6][3] = {};
    uint8_t rounds[100] = {};

    auto numLines = 0;
    std::string line;
    while (std::getline(std::cin, line)) {
        size_t pos = line.find(':');
        line = line.substr(pos + 1);

        std::istringstream ss(line);
        std::string record;
        int row = 0;

        while (std::getline(ss, record, ';') && row < 6) {
            std::istringstream recordStream(record);
            std::string word;
            int count;

            for (int i = 0; i < 3; ++i) {
                games[numLines][row][i] = 0;
            }

            while (recordStream >> count >> word) {
                if (word.ends_with(',')) word.pop_back();
                if (word == "red" || word == "red,") {
                    games[numLines][row][0] = count;
                } else if (word == "green") {
                    games[numLines][row][1] = count;
                } else if (word == "blue") {
                    games[numLines][row][2] = count;
                }
            }

            row++;
        }

        rounds[numLines] = row;
        ++numLines;
    }

    uint8_t* dGames;
    cudaMalloc(&dGames, sizeof(games));
    cudaMemcpy(dGames, games, sizeof(games), cudaMemcpyHostToDevice);

    uint8_t* dRounds;
    cudaMalloc(&dRounds, sizeof(rounds));
    cudaMemcpy(dRounds, rounds, sizeof(rounds), cudaMemcpyHostToDevice);

    uint8_t rgb[3] = {12, 13, 14};
    uint8_t* dRgb;
    cudaMalloc(&dRgb, sizeof(rgb));
    cudaMemcpy(dRgb, rgb, sizeof(rgb), cudaMemcpyHostToDevice);

    dim3 threadsPerBlock(GAME_THREADS_PER_BLOCK, ROUND_THREADS_PER_BLOCK, RGB_THREADS_PER_BLOCK);

    uint32_t* dOutPart1;
    cudaMalloc(&dOutPart1, sizeof(uint32_t) * BLOCKS);
    uint32_t* dOutPart2;
    cudaMalloc(&dOutPart2, sizeof(uint32_t) * BLOCKS);
    kernel<<<BLOCKS, threadsPerBlock>>>(dGames, dRounds, dRgb, dOutPart1, dOutPart2);
    cudaDeviceSynchronize();
    uint32_t outPart1[BLOCKS];
    cudaMemcpy(outPart1, dOutPart1, sizeof(uint32_t) * BLOCKS, cudaMemcpyDeviceToHost);
    uint32_t outPart2[BLOCKS];
    cudaMemcpy(outPart2, dOutPart2, sizeof(uint32_t) * BLOCKS, cudaMemcpyDeviceToHost);

    uint32_t totalPart1 = 0;
    for (const auto s : outPart1) totalPart1 += s;

    uint32_t totalPart2 = 0;
    for (const auto s : outPart2) totalPart2 += s;

    std::cout << totalPart1 << std::endl;
    std::cout << totalPart2 << std::endl;

    return 0;
}