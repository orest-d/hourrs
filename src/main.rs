#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;
use anyhow::Result;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Field{
    name:String,
    #[serde(rename = "type")]
    datatype:String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Schema{
    fields:Vec<Field>,
    #[serde(rename = "primaryKey")]
    primary_key:Vec<String>,
    pandas_version:String
}

#[derive(Serialize, Deserialize, Debug)]
struct HoursRecord{
    index: isize,
    rowid: isize,
    name: String,
    year: i32,
    month: i32,
    start: String,
    end: String,
    hours: String 
}

#[derive(Serialize, Deserialize, Debug)]
struct HoursDataFrame{
    schema:Schema,
    data: Vec<HoursRecord>
}

#[derive(Serialize, Deserialize, Debug)]
struct HoursData{
    dataframe:HoursDataFrame,
    names: Vec<String>,
}

impl HoursData{
    fn from_store<P: AsRef<Path>>(path:P)->Result<HoursData>{
        let file = File::open(path.as_ref().join("hours_dataframe.json"))?;
//        let mut buf_reader = BufReader::new(file);
        let dataframe: HoursDataFrame = serde_json::from_reader(
            file
        )?;
        
        let file = File::open(path.as_ref().join("hours_names.json"))?;
//        let mut buf_reader = BufReader::new(file);
        let names: Names = serde_json::from_reader(
            file
        )?;

        Ok(HoursData{
            dataframe: dataframe,
            names: names
        })
    }

    fn save<P: AsRef<Path>>(&self, path:P)->Result<()>{
       Ok(())
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Hours".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1200,
        window_height: 820,
        ..Default::default()
    }
}

type Names=Vec<String>;

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {

    let mut data = HoursData::from_store("/home/orest/PycharmProjects/hours3/app/data")?;
    println!("DF {}",serde_json::to_string(&data).unwrap());

    loop {
        clear_background(Color::from_rgba(0x12, 0x12, 0x12, 0xff));

        egui_macroquad::ui(|egui_ctx| {
            egui::SidePanel::left("left_panel").show( egui_ctx, |ui|{
                for name in data.names.iter(){
                    if ui.button(name).clicked(){
                        println!("{} clicked",name);
                    }
                }
            });
            egui::Window::new("Hours")
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                });
        });

        egui_macroquad::draw();
        next_frame().await;
    }
}