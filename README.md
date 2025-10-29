# Solana Asset Smart Contract Project

Dự án này triển khai các smart contract trên Solana để quản lý tài sản số bao gồm token và NFT với các tính năng nâng cao.

## 🎯 Mục tiêu chính

1. **Tạo và quản lý Token**: Tạo một token mới (memecoin/token cộng đồng) với khả năng mint, transfer, và burn
2. **Tính năng nâng cao**: Staking, vesting (khóa token), và phân phối token theo lịch trình
3. **NFT Collection**: Tạo và mint bộ sưu tập NFT sử dụng Metaplex Standard

## 🛠️ Công nghệ sử dụng

- **Framework**: Anchor (Solana development framework)
- **Language**: Rust cho smart contract, TypeScript cho testing
- **Standards**: SPL Token, Metaplex Token Metadata
- **Tools**: Solana CLI, Anchor CLI

## 📋 Chi tiết các bước thực hiện

### Phase 1: Thiết lập dự án và Token cơ bản

#### Bước 1.1: Chuẩn bị môi trường

```bash
# Cài đặt Solana CLI
curl -sSfL https://release.solana.com/v1.18.26/install | sh

# Cài đặt Anchor CLI
npm install -g @coral-xyz/anchor-cli

# Kiểm tra phiên bản
solana --version
anchor --version
```

#### Bước 1.2: Cấu hình Solana

```bash
# Tạo keypair (nếu chưa có)
solana-keygen new

# Cấu hình cluster (localnet để test)
solana config set --url localhost

# Tạo wallet cho test (airdrop SOL)
solana airdrop 5
```

#### Bước 1.3: Triển khai SPL Token Program

- [ ] Tạo struct `TokenManager` để quản lý thông tin token
- [ ] Implement instruction `create_token`:
  - Tạo mint account
  - Thiết lập metadata (tên, symbol, decimals)
  - Gán quyền mint authority
- [ ] Implement instruction `mint_tokens`:
  - Mint token đến ví cụ thể
  - Kiểm tra quyền mint
  - Validation số lượng
- [ ] Implement instruction `transfer_tokens`:
  - Chuyển token giữa các ví
  - Kiểm tra balance
  - Update associated token accounts
- [ ] Implement instruction `burn_tokens`:
  - Đốt token từ ví
  - Giảm total supply

### Phase 2: Tính năng Staking

#### Bước 2.1: Thiết kế Staking Pool

- [ ] Tạo struct `StakingPool`:
  ```rust
  pub struct StakingPool {
      pub authority: Pubkey,
      pub token_mint: Pubkey,
      pub reward_mint: Pubkey,
      pub reward_rate: u64,      // Reward per second
      pub total_staked: u64,
      pub bump: u8,
  }
  ```

#### Bước 2.2: Implement Staking Instructions

- [ ] `initialize_staking_pool`: Tạo pool staking mới
- [ ] `stake_tokens`: Stake token vào pool
  - Chuyển token từ user vào pool
  - Tạo stake account cho user
  - Tính toán thời gian bắt đầu stake
- [ ] `unstake_tokens`: Rút token đã stake
  - Tính toán reward
  - Chuyển token + reward về user
  - Đóng stake account
- [ ] `claim_rewards`: Claim reward mà không unstake
  - Tính toán reward tích lũy
  - Mint reward token cho user
  - Reset reward timer

#### Bước 2.3: Reward Calculation System

- [ ] Implement công thức tính reward theo thời gian
- [ ] Xử lý compound reward (nếu có)
- [ ] Thiết lập minimum staking time

### Phase 3: Token Vesting (Khóa Token)

#### Bước 3.1: Vesting Schedule Design

- [ ] Tạo struct `VestingSchedule`:
  ```rust
  pub struct VestingSchedule {
      pub beneficiary: Pubkey,
      pub total_amount: u64,
      pub start_time: i64,
      pub cliff_time: i64,        // Thời gian cliff
      pub duration: i64,          // Tổng thời gian vest
      pub released_amount: u64,   // Đã claim
  }
  ```

#### Bước 3.2: Implement Vesting Instructions

- [ ] `create_vesting_schedule`: Tạo lịch trình vest
- [ ] `release_vested_tokens`: Claim token đã vest
  - Tính toán số token có thể claim
  - Kiểm tra cliff time
  - Update released amount
- [ ] `revoke_vesting`: Thu hồi vesting (chỉ admin)

### Phase 4: NFT Collection với Metaplex

#### Bước 4.1: Setup Metaplex Dependencies

```toml
# Thêm vào Cargo.toml
[dependencies]
mpl-token-metadata = "1.13.2"
```

#### Bước 4.2: Collection Setup

- [ ] Tạo struct `NftCollection`:
  ```rust
  pub struct NftCollection {
      pub authority: Pubkey,
      pub collection_mint: Pubkey,
      pub collection_metadata: Pubkey,
      pub collection_master_edition: Pubkey,
      pub total_supply: u32,
      pub minted_count: u32,
  }
  ```

#### Bước 4.3: Implement NFT Instructions

- [ ] `create_collection`: Tạo collection NFT
  - Tạo collection mint
  - Thiết lập metadata cho collection
  - Tạo master edition
- [ ] `mint_nft`: Mint NFT từ collection
  - Tạo NFT mint account
  - Thiết lập metadata
  - Verify collection membership
  - Tạo master edition cho NFT
- [ ] `update_nft_metadata`: Cập nhật metadata NFT
- [ ] `transfer_nft`: Chuyển NFT giữa các ví

### Phase 5: Testing và Deployment

#### Bước 5.1: Unit Tests

- [ ] Test token creation và basic operations
- [ ] Test staking flow (stake/unstake/claim)
- [ ] Test vesting schedule
- [ ] Test NFT minting và transfer
- [ ] Test edge cases và error handling

#### Bước 5.2: Integration Tests

- [ ] Test tương tác giữa các modules
- [ ] Test với multiple users
- [ ] Performance testing
- [ ] Security testing

#### Bước 5.3: Deployment

```bash
# Build program
anchor build

# Deploy lên localnet
anchor deploy

# Deploy lên devnet
solana config set --url devnet
anchor deploy

# Deploy lên mainnet (production)
solana config set --url mainnet-beta
anchor deploy
```

## 🧪 Chạy Tests

```bash
# Chạy tất cả tests
anchor test

# Chạy test cụ thể
anchor test --skip-local-validator tests/token.ts

# Test với logging
ANCHOR_LOG=true anchor test
```

## 📁 Cấu trúc dự án

```
asset/
├── programs/asset/src/
│   ├── lib.rs              # Entry point
│   ├── instructions/       # Các instruction modules
│   │   ├── mod.rs
│   │   ├── token.rs        # Token operations
│   │   ├── staking.rs      # Staking operations
│   │   ├── vesting.rs      # Vesting operations
│   │   └── nft.rs          # NFT operations
│   ├── state/              # Data structures
│   │   ├── mod.rs
│   │   ├── token.rs
│   │   ├── staking.rs
│   │   ├── vesting.rs
│   │   └── nft.rs
│   └── error.rs            # Custom errors
├── tests/                  # Test files
└── app/                    # Frontend (nếu có)
```

## 🔐 Security Considerations

1. **Access Control**: Kiểm tra quyền trước khi thực hiện operations
2. **Validation**: Validate tất cả input parameters
3. **Overflow Protection**: Sử dụng checked arithmetic
4. **Account Verification**: Verify account ownership và signatures
5. **Rate Limiting**: Implement rate limiting cho các operations nhạy cảm

## 📚 Tài liệu tham khảo

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [SPL Token Documentation](https://spl.solana.com/token)
- [Metaplex Documentation](https://docs.metaplex.com/)

## 🚀 Bước tiếp theo

Sau khi hoàn thành dự án này, bạn có thể mở rộng với:

- Governance system cho DAO
- Liquidity pool và AMM
- Cross-chain bridge
- Advanced DeFi protocols
