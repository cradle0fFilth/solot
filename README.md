# Solot

## 初始化测试

1. 使用 Solana CLI 导出程序：
   ```bash
   solana program dump -u m metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so

2. 在本地运行 Solana 测试验证器：
   ```bash
   solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so

3. 使用 Anchor 运行测试（跳过本地验证器):
   ```bash
   anchor test --skip-local-validator
