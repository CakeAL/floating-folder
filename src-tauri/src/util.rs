use std::{io::Cursor, os::windows::ffi::OsStrExt, path::Path, ptr::null_mut};

use base64::{engine::general_purpose, Engine};
use image::{ImageBuffer, Rgba};
use winapi::{
    shared::windef::{HBITMAP, HDC, HICON},
    um::{
        shellapi::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON},
        wingdi::{
            CreateCompatibleDC, DeleteDC, GetDIBits, GetObjectW, BITMAP, BITMAPINFO,
            BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, RGBQUAD,
        },
        winuser::{DestroyIcon, GetIconInfo, ICONINFO},
    },
};

pub fn get_icon_base64(path: impl AsRef<Path>) -> Option<String> {
    let mut shfileinfo: SHFILEINFOW = unsafe { std::mem::zeroed() };
    let wide_path: Vec<u16> = path
        .as_ref()
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    // 使用 SHGetFileInfoW 获取图标句柄
    let png_data;
    unsafe {
        SHGetFileInfoW(
            wide_path.as_ptr(),
            0,
            &mut shfileinfo,
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON,
        );
        if shfileinfo.hIcon.is_null() {
            return None;
        }

        let hicon: HICON = shfileinfo.hIcon;
        let hbitmap: HBITMAP = icon_to_bitmap(hicon)?;
        png_data = bitmap_to_png(hbitmap)?;

        DestroyIcon(shfileinfo.hIcon);
    }
    Some(format!(
        "data:image/png;base64,{}",
        general_purpose::STANDARD.encode(png_data)
    ))
}

// HICON 转换为 HBITMAP
fn icon_to_bitmap(hicon: HICON) -> Option<HBITMAP> {
    let mut icon_info: ICONINFO = unsafe { std::mem::zeroed() };
    if unsafe { GetIconInfo(hicon, &mut icon_info) } == 0 {
        return None;
    }
    Some(icon_info.hbmColor)
}

// HBITMAP 转换为 PNG
fn bitmap_to_png(hbitmap: HBITMAP) -> Option<Vec<u8>> {
    unsafe {
        // 创建 DC
        let hdc: HDC = CreateCompatibleDC(null_mut());
        if hdc.is_null() {
            return None;
        }
        // 获取位图信息
        let mut bitmap: BITMAP = std::mem::zeroed();
        if GetObjectW(
            hbitmap as _,
            std::mem::size_of::<BITMAP>() as i32,
            &mut bitmap as *mut _ as _,
        ) == 0
        {
            DeleteDC(hdc);
            return None;
        }
        // 设置位图信息
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: bitmap.bmWidth,
                biHeight: -bitmap.bmHeight, // 使用负值以将图像设置为顶向下
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }; 1],
        };

        let bmp_size = (bitmap.bmWidth * bitmap.bmHeight * 4) as usize;
        let mut pixel_data = vec![0u8; bmp_size];
        // 获取位图信息
        if GetDIBits(
            hdc,
            hbitmap,
            0,
            bitmap.bmHeight as u32,
            pixel_data.as_mut_ptr() as *mut _,
            &mut bmi,
            DIB_RGB_COLORS,
        ) == 0
        {
            DeleteDC(hdc);
            return None;
        }
        // 纠正颜色通道顺序（BGRA -> RGBA）
        for chunk in pixel_data.chunks_exact_mut(4) {
            let b = chunk[0];
            let r = chunk[2];
            chunk[0] = r;
            chunk[2] = b;
        }
        // 使用 image 库转换像素数据为 png
        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(
            bitmap.bmWidth as u32,
            bitmap.bmHeight as u32,
            pixel_data,
        )?;
        let mut png_data = Vec::new();
        if image::DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut png_data), image::ImageFormat::Png)
            .is_err()
        {
            DeleteDC(hdc);
            return None;
        }
        DeleteDC(hdc);
        Some(png_data)
    }
}

#[cfg(test)]
mod tests {
    use super::get_icon_base64;

    #[test]
    fn test_get_icon_base64() {
        let lnk_path =
            "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Microsoft Edge.lnk";
        let res = get_icon_base64(lnk_path);
        dbg!(res);
    }
}
