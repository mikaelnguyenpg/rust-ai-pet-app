"use client";

import dynamic from "next/dynamic";

// import AiClassifier from "./AiClassifier";
const AiClassifier = dynamic(() => import("./AiClassifier"), {
  ssr: false,
  loading: () => <p>Đang khởi tạo hệ thống AI...</p>,
});

export default function AiWrapper() {
  return <AiClassifier />;
}
