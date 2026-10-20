use std::env;


struct Batch {
batch_id: u32,
start_date: DateTime,
inventory: Vec, //crop + quantity + stage
}
//maybe a tuple //I wonder if searching by crop type will matter
//isolating indivual crop does not decrease crop quantity
//removing indivual crop does



struct CropInfo {
name: String,
cultivar: String,
ideal_PH: f32,
nutrient_mixu: u32, 
estimated_harvest_time: String //possibly datetime if I want to derive dates from here
light/shade_requirements: LightRequirements,
}

Enum LightRequirements /* I can make these structs if I wanted to store some info */{
    HighSun,
    PartialSun,
    LowSun,
}

struct Crop {
CropInfo, //may be unneccesary
}

struct Plant 
crop: Crop
start_date: DateTime //&Batch.start_date
living: bool
stage: Stage //germination, seedling, planted, havested

enum Stage{
    Germination,
    Seedling,
    Planted,
    Havested,
}

//maybe harvest is a method batch has instead of it's own data structure?
struct Harvest //I'm not so sure about this implementation
date: DateTime
crops_harvested: u32
crops_lost: u32 
total_lost //This might be better derived
total_harvested //This might be better derived


fn main() {
    println!("Hello, world!");
}

impl Batch {
    fn isolate_plant () {}
    /* create independent record for plant from batch
    Created records decrease batch inventory and are tracked seperately
    until remerged */

    fn merge_plant () {}
    //re-add seperated plant into batch inventory to be tracked by batch


}

impl Harvest {
    fn crops_harvested() {}
    //total crop inventory 

    fn crops_lost() {}
}


fn main () {
    todo!();
}
