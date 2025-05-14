# 反编译分析

## 生成一个例子，来测试反编译

下载zksync的[zksolc编译器](https://github.com/matter-labs/era-compiler-solidity/releases/tag/1.5.13)
这个需要依赖[solc fork的二进制](https://github.com/matter-labs/era-solidity/releases)
```shell
./zksolc --solc solc-macosx-arm64-0.8.29-1.0.1 --asm demo0.sol
./zksolc --solc solc-macosx-arm64-0.8.29-1.0.1 --bin demo0.sol
```