
### 生成不同格式文件
`cargo run -- csv -i ./assets/juventus.csv --format json`

### 生成32位密码
`cargo run -- genpass -l 32 > out.txt`

### 生成base64编码
`cargo run -- base64`

### 生成签名
`cargo run -- text sign -k fixtures/blake3.txt`
`cargo run -- text sign -k fixtures/blake3.txt --format ed25519`

### 验证签名
`cargo run -- text sign -k fixtures/blake3.txt`

### 生成随机的
`cargo run -- text generate -o fixtures`
