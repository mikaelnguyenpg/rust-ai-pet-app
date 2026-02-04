"use client";

import { Divide } from "lucide-react";
import { useEffect, useState } from "react";

export default function AiClassifier() {
  const [result, setResult] = useState<string>("Đang chờ ảnh...");
  const [loading, setLoading] = useState(false);
  const [wasm, setWasm] = useState<any>(null);

  // 1. Load module WASM khi trang web vừa mở
  useEffect(() => {
    const loadWasm = async () => {
      try {
        // 'ai-engine' là tên package bạn đã link
        const module = await import("ai-engine");
        // Khởi tạo module (bắt buộc với --target web)
        await module.default();
        setWasm(module);
        console.log("Great! Loaded ai-engine");
      } catch (err) {
        console.error("Lỗi load WASM:", err);
      }
    };
    loadWasm();
  }, []);

  const handleUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file || !wasm) return;

    setLoading(true);
    setResult("Rust đang phân tích...");

    try {
      // 2. Đọc file ảnh thành ArrayBuffer
      const imageBuffer = await file.arrayBuffer();
      const imageArray = new Uint8Array(imageBuffer);

      // 3. Đọc file model .onnx từ thư mục public
      const modelResponse = await fetch("/model.onnx");
      const modelBuffer = await modelResponse.arrayBuffer();
      const modelArray = new Uint8Array(modelBuffer);

      // 4. Gọi hàm Rust đã viết ở Bước 4
      const prediction = wasm.classify_pet(imageArray, modelArray);
      setResult(prediction);
    } catch (err) {
      setResult("Lỗi xử lý: " + err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-8 bg-white rounded-2xl shadow-xl w-full max-w-md">
      <h1 className="text-2xl font-bold mb-6 text-center">
        AI Chó & Mèo (Rust WASM)
      </h1>

      <div className="border-2 border-dashed border-blue-400 rounded-lg p-10 text-center mb-6">
        <input
          type="file"
          accept="image/*"
          onChange={handleUpload}
          className="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
        />
      </div>

      <div className="bg-gray-50 p-4 rounded-lg">
        <p className="text-sm text-gray-500 uppercase tracking-wider font-semibold">
          Kết quả:
        </p>
        <p
          className={`text-xl font-mono mt-2 ${loading ? "animate-pulse text-blue-600" : "text-green-600"}`}
        >
          {result}
        </p>
      </div>
    </div>
  );
}
