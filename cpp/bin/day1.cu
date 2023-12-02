// 1000 lines
// max 52 chars

// Kernel definition
#include <cstring>
#include <string>
#include <iostream>

// in[1024][64], out[1024]

static const int ROWS = 1024;
static const int MAX_CHARS = 64;
static const int MAX_SEQ_LEN = 8;

static const int BLOCKS = 32;
static const int ROW_THREADS = 32;
static const int COL_THREADS = 32;


__global__ void part2Kernel(const char *in, const char *seqs, const uint8_t *values, uint32_t *out, unsigned int numRows, unsigned int numSeqs)
{
    __shared__ int32_t sharedIndexes[32][32];
    __shared__ int32_t sharedMaxIndexes[32][32];
    __shared__ uint32_t sharedArgMin[32][32];
    __shared__ uint32_t sharedArgMax[32][32];
    __shared__ uint32_t sharedSums[32];

    // Initialise shared memory
    sharedIndexes[threadIdx.x][threadIdx.y] = -1;
    sharedMaxIndexes[threadIdx.x][threadIdx.y] = -1;
    sharedArgMin[threadIdx.x][threadIdx.y] = threadIdx.y;
    sharedArgMax[threadIdx.x][threadIdx.y] = threadIdx.y;
    if (threadIdx.y == 0) sharedSums[threadIdx.x] = 0;
    __syncthreads();

    auto rowIdx = blockDim.x * blockIdx.x + threadIdx.x;
    auto seqIdx = threadIdx.y;

    // Nothing to do for threads outside rowIdx or seqIdx
    if (rowIdx >= numRows) return;
    if (seqIdx >= numSeqs) return;


    // Compute first and last occurrences (if any) of each sequence
    {
        auto i = 0;
        while (i < MAX_CHARS) {
            auto j = 0;
            while ((i + j < MAX_CHARS) && in[rowIdx * MAX_CHARS + i + j] != 0 && in[rowIdx * MAX_CHARS + i + j] == seqs[seqIdx * MAX_SEQ_LEN + j]) {
                ++j;
            }

            // Check if entire token has been found
            if (seqs[seqIdx * MAX_SEQ_LEN + j] == 0) {
                sharedIndexes[threadIdx.x][seqIdx] = i;
                break;
            }

            ++i;
        }
    }

    {
        auto i = MAX_CHARS - 1;
        while (i >= 0) {
            auto j = 0;
            while ((i + j < MAX_CHARS) && in[rowIdx * MAX_CHARS + i + j] != 0 && in[rowIdx * MAX_CHARS + i + j] == seqs[seqIdx * MAX_SEQ_LEN + j]) {
                ++j;
            }

            // Check if entire token has been found
            if (seqs[seqIdx * MAX_SEQ_LEN + j] == 0) {
                sharedMaxIndexes[threadIdx.x][seqIdx] = i;
                break;
            }

            --i;
        }
    }

    __syncthreads();

    // Compute argmins and argmaxes
    for (auto s = COL_THREADS >> 1; s > 0; s >>= 1) {
        if (seqIdx < s) {
            auto leftArgMinIdx = sharedArgMin[threadIdx.x][seqIdx];
            auto rightArgMinIdx = sharedArgMin[threadIdx.x][seqIdx + s];

            auto leftArgMinCandidate = sharedIndexes[threadIdx.x][leftArgMinIdx];
            auto rightArgMinCandidate = sharedIndexes[threadIdx.x][rightArgMinIdx];

            if ((leftArgMinCandidate < 0 && rightArgMinCandidate < 0) || (leftArgMinCandidate >= 0 && rightArgMinCandidate < 0))  {
                sharedArgMin[threadIdx.x][seqIdx] = leftArgMinIdx;
            } else if (leftArgMinCandidate < 0) {
                sharedArgMin[threadIdx.x][seqIdx] = rightArgMinIdx;
            } else if (leftArgMinCandidate < rightArgMinCandidate) {
                sharedArgMin[threadIdx.x][seqIdx] = leftArgMinIdx;
            } else {
                sharedArgMin[threadIdx.x][seqIdx] = rightArgMinIdx;
            }

            auto leftArgMaxIdx = sharedArgMax[threadIdx.x][seqIdx];
            auto rightArgMaxIdx = sharedArgMax[threadIdx.x][seqIdx + s];

            auto leftArgMaxCandidate = sharedMaxIndexes[threadIdx.x][leftArgMaxIdx];
            auto rightArgMaxCandidate = sharedMaxIndexes[threadIdx.x][rightArgMaxIdx];

            if ((leftArgMaxCandidate < 0 && rightArgMaxCandidate < 0) || (leftArgMaxCandidate >= 0 && rightArgMaxCandidate < 0)) {
                sharedArgMax[threadIdx.x][seqIdx] = leftArgMaxIdx;
            } else if (leftArgMaxCandidate < 0) {
                sharedArgMax[threadIdx.x][seqIdx] = rightArgMaxIdx;
            } else if (leftArgMaxCandidate < rightArgMaxCandidate) {
                sharedArgMax[threadIdx.x][seqIdx] = rightArgMaxIdx;
            } else {
                sharedArgMax[threadIdx.x][seqIdx] = leftArgMaxIdx;
            }
        }
        __syncthreads();
    }

    // Compute partial sum for block
    if (seqIdx == 0) {
        auto a = 10 * values[sharedArgMin[threadIdx.x][0]] + values[sharedArgMax[threadIdx.x][0]];
        sharedSums[threadIdx.x] = a;
        __syncthreads();

        // Compute sum of computed values in block
        for (auto s = ROW_THREADS >> 1; s > 0; s >>= 1) {
            if (threadIdx.x < s) {
                sharedSums[threadIdx.x] += sharedSums[threadIdx.x + s];
            }
            __syncthreads();
        }

        if (threadIdx.x == 0) out[blockIdx.x] = sharedSums[0];
    }
}


int main() {
    // Input buffer
    auto in = (char *) malloc(ROWS * MAX_CHARS);
    std::fill(in, in + (ROWS * MAX_CHARS), static_cast<uint8_t>(0));

    auto i = 0;
    std::string line;
    while (std::getline(std::cin, line)) {
        auto j = 0;
        for (const auto c : line) {
            in[MAX_CHARS * i + j] = c;
            ++j;
        }
        ++i;
    }

    char* dIn;
    cudaMalloc(&dIn, sizeof(uint8_t) * ROWS * MAX_CHARS);
    cudaMemcpy(dIn, in, sizeof(uint8_t) * ROWS * MAX_CHARS, cudaMemcpyHostToDevice);

    // All possible strings to search
    const int numStrings = 19;
    const int maxLen = 8;
    char hostStrings[numStrings][maxLen];

    strcpy(hostStrings[0], "0");
    strcpy(hostStrings[1], "1");
    strcpy(hostStrings[2], "2");
    strcpy(hostStrings[3], "3");
    strcpy(hostStrings[4], "4");
    strcpy(hostStrings[5], "5");
    strcpy(hostStrings[6], "6");
    strcpy(hostStrings[7], "7");
    strcpy(hostStrings[8], "8");
    strcpy(hostStrings[9], "9");
    strcpy(hostStrings[10], "one");
    strcpy(hostStrings[11], "two");
    strcpy(hostStrings[12], "three");
    strcpy(hostStrings[13], "four");
    strcpy(hostStrings[14], "five");
    strcpy(hostStrings[15], "six");
    strcpy(hostStrings[16], "seven");
    strcpy(hostStrings[17], "eight");
    strcpy(hostStrings[18], "nine");

    char* dStrings;
    cudaMalloc((void**)&dStrings, numStrings * maxLen * sizeof(char));
    cudaMemcpy(dStrings, hostStrings, numStrings * maxLen * sizeof(char), cudaMemcpyHostToDevice);

    // Lookup table for tokens by their index
    uint8_t hValues[numStrings] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    uint8_t *dValues;
    cudaMalloc(&dValues, numStrings * sizeof(uint8_t));
    cudaMemcpy(dValues, hValues, numStrings * sizeof(uint8_t), cudaMemcpyHostToDevice);

    // Memory for each block to write its partial sum to
    uint32_t* dOutPart1;
    cudaMalloc(&dOutPart1, BLOCKS * sizeof(uint32_t));

    uint32_t* dOutPart2;
    cudaMalloc(&dOutPart2, BLOCKS * sizeof(uint32_t));

    // Launch kernels
    dim3 threadsPerBlock(ROW_THREADS, COL_THREADS);
    part2Kernel<<<BLOCKS, threadsPerBlock>>>(dIn, dStrings, dValues, dOutPart1, i, 10);
    part2Kernel<<<BLOCKS, threadsPerBlock>>>(dIn, dStrings, dValues, dOutPart2, i, 19);
    cudaDeviceSynchronize();

    uint32_t outPart1[BLOCKS];
    cudaMemcpy(outPart1, dOutPart1, BLOCKS * sizeof(uint32_t), cudaMemcpyDeviceToHost);

    uint32_t outPart2[BLOCKS];
    cudaMemcpy(outPart2, dOutPart2, BLOCKS * sizeof(uint32_t), cudaMemcpyDeviceToHost);

    uint32_t totalPart1 = 0;
    for (auto m : outPart1) {
        totalPart1 += m;
    }

    uint32_t totalPart2 = 0;
    for (auto m : outPart2) {
        totalPart2 += m;
    }

    std::cout << totalPart1 << std::endl;
    std::cout << totalPart2 << std::endl;

    cudaFree(dIn);
    cudaFree(dStrings);
    cudaFree(dValues);
    cudaFree(dOutPart1);

    return 0;
}