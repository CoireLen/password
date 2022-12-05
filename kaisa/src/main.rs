#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::{Context,FontDefinitions,FontData,FontFamily};
use core::num;
fn main() {
    let mut options = eframe::NativeOptions::default();

    eframe::run_native(
        "凯撒密码",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}

struct MyApp {
    mingwen: String,//未加密的文字
    pianyi:i32,//偏移值-26，26
    miwen:String,//加密后的文字
}

impl MyApp {
    fn new() -> Self {
        Self {
            mingwen: "".to_owned(),
            miwen: "".to_owned(),
            pianyi:0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("syht".to_owned(),
        FontData::from_static(include_bytes!("/usr/share/fonts/truetype/SourceHanSansCN-Heavy.otf")));
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "syht".to_owned());
        ctx.set_fonts(fonts);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("凯撒密码加密解密");
            ui.horizontal(|ui| {
                ui.label("需要加密的文字: ");
                ui.text_edit_multiline(&mut self.mingwen);
            });
            ui.add(egui::Slider::new(&mut self.pianyi, -26..=26).text("偏移"));
            self.miwen.clear();
            for i in self.mingwen.as_bytes(){
                if i.is_ascii_alphanumeric(){
                    if i>=&('a' as u8) &&i<=&('z' as u8){
                        let mut x:i32=*i as i32-'a' as i32;
                        x=x+self.pianyi;
                        if x<0{
                            x+=26;
                        }
                        x=x%26;
                        self.miwen.push((x as u8+'a' as u8)as char);
                    }
                    else if  i>=&('A' as u8) &&i<=&('Z' as u8){
                        let mut x:i32=*i as i32-'A' as i32;
                        x=x+self.pianyi;
                        if x<0{
                            x+=26;
                        }
                        x=x%26;
                        self.miwen.push((x as u8+'A' as u8)as char);
                    }
                }
                else{
                    self.miwen.push(*i as char);
                }
            }
            ui.horizontal(|ui| {
                ui.label("加密后的文字: ");
                ui.text_edit_multiline(&mut self.miwen);
            });
        });
    }
}