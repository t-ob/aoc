// 1000 lines
// max 52 chars

// Kernel definition
#include <string>
#include <iostream>

// in[1024][64], out[1024]

static const int ROWS = 1024;
static const int COLS = 64;
static const int THREADS_PER_BLOCK = 256;

__global__ void part1Kernel(uint8_t *in, uint8_t *out, unsigned int numRows)
{
    auto i = blockDim.x * blockIdx.x + threadIdx.x;
    if (i >= numRows) return;

    uint8_t nums[2];

    auto k = 0;

    for (auto j = 0; j < 64; ++j) {
        auto c = in[COLS * i + j];
        if (c < '0' || c > '9') continue;

        if (k == 0) {
            nums[0] = c - '0';
            nums[1] = c - '0';
            k = 1;
        } else {
            nums[1] = c - '0';
        }
    }

    out[i] = 10 * nums[0] + nums[1];
}

__global__ void part2Kernel(uint8_t *in, int8_t *out, unsigned int numRows)
{
    auto i = blockDim.x * blockIdx.x + threadIdx.x;
    if (i >= numRows) return;

    uint8_t nums[2];

    auto k = 0;
    auto l = in[COLS * i + COLS - 1];

    for (auto j = 0; j < 63; ++j) {
        auto c = in[COLS * i + j];
        if (c < '0' || c > '9') continue;

        if (k == 0) {
            nums[0] = c - '0';
            nums[1] = c - '0';
            k = 1;
        } else {
            nums[1] = c - '0';
        }
    }

    out[i] = 10 * nums[0] + nums[1];
}

__global__ void sumKernel(uint8_t *input, uint32_t *partialSums, int size) {
    __shared__ uint32_t shared[THREADS_PER_BLOCK];
    auto tid = threadIdx.x;
    auto i = blockIdx.x * blockDim.x + threadIdx.x;

    shared[tid] = (i < size) ? input[i] : 0;
    __syncthreads();

    for (auto s = blockDim.x / 2; s > 0; s >>= 1) {
        if (tid < s) {
            shared[tid] += shared[tid + s];
        }
        __syncthreads();
    }

    if (tid == 0) partialSums[blockIdx.x] = shared[0];
}



int main() {
    auto in = (uint8_t *) malloc(ROWS * COLS);
    auto out = (uint8_t *) malloc(ROWS);
    std::fill(in, in + (ROWS * COLS), static_cast<uint8_t>(0));
    std::fill(out, out + ROWS, static_cast<uint8_t>(0));

    auto i = 0;

    std::string line;
    while (std::getline(std::cin, line)) {
        auto j = 0;
        for (const auto c : line) {
            in[COLS * i + j] = c;
            ++j;
        }
        ++i;
    }

    uint8_t* dIn;
    cudaMalloc(&dIn, sizeof(uint8_t) * ROWS * COLS);
    uint8_t* dOut;
    cudaMalloc(&dOut, sizeof(uint8_t) * ROWS);


    cudaMemcpy(dIn, in, sizeof(uint8_t) * ROWS * COLS, cudaMemcpyHostToDevice);

    auto blocks = (ROWS + THREADS_PER_BLOCK - 1) / THREADS_PER_BLOCK;
    part1Kernel<<<blocks, THREADS_PER_BLOCK>>>(dIn, dOut, i);

    uint32_t *dPartialSums;
    cudaMalloc((void**)&dPartialSums, blocks * sizeof(uint32_t));

    sumKernel<<<blocks, THREADS_PER_BLOCK>>>(dOut, dPartialSums, i);

    uint32_t partialSums[blocks];
    cudaMemcpy(partialSums, dPartialSums, blocks * sizeof(uint32_t), cudaMemcpyDeviceToHost);

    uint32_t totalSum = 0;
    for (auto b = 0; b < blocks; b++) {
        totalSum += partialSums[b];
    }

    std::cout << std::to_string(totalSum) << std::endl;

    cudaFree(dIn);
    cudaFree(dOut);
    cudaFree(dPartialSums);

    return 0;
}