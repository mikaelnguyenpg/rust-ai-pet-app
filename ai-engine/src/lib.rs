use wasm_bindgen::prelude::*;
use tract_onnx::prelude::*;
use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::GenericImageView;

#[wasm_bindgen]
pub fn classify_pet(image_data: &[u8], model_data: &[u8]) -> Result<String, JsValue> {
    // 1. Giải mã ảnh từ mảng byte do JavaScript gửi lên
    let img = ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()
        .map_err(|_| "Không đọc được định dạng ảnh")?
        .decode()
        .map_err(|_| "Lỗi giải mã ảnh")?;

    // 2. Tiền xử lý: Resize về 224x224 (kích thước chuẩn của MobileNet)
    let resized = img.resize_exact(224, 224, image::imageops::FilterType::Triangle);
    
    // Chuyển ảnh thành Tensor (RGB, 3 kênh, giá trị từ 0.0 - 1.0)
    let tensor: Tensor = tract_ndarray::Array4::from_shape_fn((1, 3, 224, 224), |(_, c, y, x)| {
        let pixel = resized.get_pixel(x as u32, y as u32);
        // let channel_value = match c {
        //     0 => pixel[0], // R
        //     1 => pixel[1], // G
        //     2 => pixel[2], // B
        //     _ => 0,
        // };
        // (channel_value as f32 / 255.0 - 0.5) / 0.5 // Normalize đơn giản
        let v = pixel[c] as f32 / 255.0;
        match c {
            0 => (v - 0.485) / 0.229, // Chuẩn hóa kênh Đỏ
            1 => (v - 0.456) / 0.224, // Chuẩn hóa kênh Xanh lá
            2 => (v - 0.406) / 0.225, // Chuẩn hóa kênh Xanh dương
            _ => 0.0,
        }
    }).into();

    // 3. Load Model và Chạy Inference (Suy luận)
    // model_data được truyền từ JS (đọc từ file .onnx)
    let mut model_cursor = Cursor::new(model_data);
    let model = onnx()
        .model_for_read(&mut model_cursor)
        .map_err(|_| "Lỗi load model ONNX")?
        .into_optimized()
        .map_err(|_| "Lỗi tối ưu model")?
        .into_runnable()
        .map_err(|_| "Lỗi tạo runnable model")?;

    let result = model.run(tvec!(tensor.into()))
        .map_err(|e| JsValue::from_str(&format!("Lỗi khi chạy model: {:?}", e)))?;

    // 4. Xử lý kết quả (Softmax & Labeling)
    let logits = result[0].to_array_view::<f32>().unwrap();
    
    // Tìm Max Logit (Giá trị thô lớn nhất)
    let (max_idx, max_logit) = logits.iter().enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    // Tính Softmax cho giá trị Max để ra % thực tế
    let exp_sum: f32 = logits.iter().map(|x| x.exp()).sum();
    let confidence = (max_logit.exp() / exp_sum) * 100.0;

    // Phân loại nhãn theo ImageNet (Mèo thường là 281-285, Chó là 151-268)
    let pet_type = match max_idx {
        151..=268 => "Chó 🐶",
        281..=285 => "Mèo 🐱",
        _ => "Vật thể khác 📦",
    };

    Ok(format!("{} (Độ tin cậy: {:.2}%)", pet_type, confidence))

    // // 4. Lấy kết quả (Index cao nhất)
    // let probabilities = result[0].to_array_view::<f32>()
    //     .map_err(|_| "Lỗi trích xuất kết quả")?;

    // // a. Tìm Index có giá trị lớn nhất (Argmax)
    // let mut max_idx = 0;
    // let mut max_val = f32::MIN;
    // for (i, &val) in probabilities.iter().enumerate() {
    //     if val > max_val {
    //         max_val = val;
    //         max_idx = i;
    //     }
    // }

    // // b. Tính Softmax đơn giản để lấy % (chỉ cần tính cho giá trị max để demo)
    // // Trong thực tế cần tính: exp(x) / sum(exp(all_x))
    // let confidence = (max_val.exp() / probabilities.iter().map(|x| x.exp()).sum::<f32>()) * 100.0;

    // // c. Phân loại dựa trên ImageNet Index (Dải index của giống loài)
    // // Index 151-268: Dogs | Index 281-285: Cats
    // let label = match max_idx {
    //     151..=268 => format!("Chó 🐶(Giống index: {})", max_idx),
    //     281..=285 => format!("Mèo 🐱(Giống index: {})", max_idx),
    //     _ => format!("Vật thể khác 📦(Index: {})", max_idx),
    // };

    // Ok(format!("{} - Độ tin cậy: {:.2}%", label, confidence))
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
