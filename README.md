This is a [Next.js](https://nextjs.org) project bootstrapped with [`create-next-app`](https://nextjs.org/docs/app/api-reference/cli/create-next-app).

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.

## 1. Techstack chi tiết

Ngôn ngữ: Rust (Logic & AI Inference) + TypeScript (Frontend).
Frontend Framework: Next.js (App Router).
AI Library (Rust): Tract (Cực nhẹ để chạy mô hình ONNX trên CPU/WASM).
Công cụ biên dịch: wasm-pack (Đóng gói Rust sang WebAssembly).
Môi trường: Ubuntu (LTS 22.04 hoặc 24.04).

## 2. Các bước thiết lập trên Ubuntu

### Bước 1: Cài đặt môi trường phát triển

```bash
# 1. Cài đặt Node.js (dùng nvm)
curl -o- https://raw.githubusercontent.com | bash
source ~/.bashrc
nvm install 20

# 2. Cài đặt Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Cài đặt wasm-pack
curl https://rustwasm.github.io -sSf | sh
```

### Bước 2: Khởi tạo dự án Next.js

```bash
npx create-next-app@latest ai-pet-app --typescript --tailwind --eslint
cd ai-pet-app
```

### Bước 3: Tạo Module AI bằng Rust

```bash
cargo new --lib ai-engine
cd ai-engine
```

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
tract-onnx = "0.21" # Thư viện chạy file ONNX
image = "0.24"      # Xử lý ảnh
getrandom = { version = "0.2", features = ["js"] }
```

### Bước 4: Viết logic nhận diện (Tóm tắt)

1. Chuẩn bị File Model AI

- Lưu file vào: ai-pet-app/public/model.onnx

2. Code chi tiết ai-engine/src/lib.rs

### Bước 5: Biên dịch Rust sang WASM

```bash
wasm-pack build --target web
```

Để sử dụng nó trong Next.js, hãy thực hiện bước "link" thư viện này để Next.js nhận diện được:

1. Tại thư mục ai-engine/pkg: Chạy lệnh `npm link`
2. Tại thư mục gốc ai-pet-app: Chạy lệnh `npm link ai-engine` (thay ai-engine bằng tên project trong file Cargo.toml của bạn).

### Bước 6: Tích hợp vào Next.js

### Bước 7: Chạy Local

```bash
# Tại thư mục gốc ai-pet-app
npm run dev
```
