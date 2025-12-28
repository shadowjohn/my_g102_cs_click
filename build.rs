use base64::Engine;

fn main() {
    #[cfg(windows)]
    {
        use std::fs::File;
        use std::path::Path;

        let b64 = include_str!("assets/icon_base64.txt").trim();
        let png = base64::engine::general_purpose::STANDARD
            .decode(b64)
            .expect("base64 decode failed");

        let img = image::load_from_memory(&png).expect("invalid image");
        let img = img.resize_exact(256, 256, image::imageops::Lanczos3);

        // 轉成 ico
        let icon = ico::IconImage::from_rgba_data(256, 256, img.to_rgba8().into_raw());
        let mut dir = ico::IconDir::new(ico::ResourceType::Icon);
        dir.add_entry(ico::IconDirEntry::encode(&icon).unwrap());

        std::fs::create_dir_all("target").unwrap();
        let ico_path = Path::new("target/icon.ico");
        let mut f = File::create(ico_path).unwrap();
        dir.write(&mut f).unwrap();

        // 產生 rc
        let rc_path = "target/icon.rc";
        std::fs::write(rc_path, r#"IDI_ICON1 ICON "target/icon.ico""#).unwrap();

        // embed
        embed_resource::compile(rc_path, embed_resource::NONE);
    }
}