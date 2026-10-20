use helpers::calendar;
use std::collections::HashMap;



#[derive(Debug)]
struct Batch {
    batch_id: u32,
    start_date: String,
    inventory: HashMap<String, InventoryItem>
}


struct Crops {}
// inventory moved to crop
struct IsolatedCrops; // crops tracked independent from batch
struct AvailableCrops;  //struct containing every available crop I farm


#[derive(Debug)]
struct InventoryItem {
    name: String,
    quantity: u32,
    stage: String,
    isolated: u32,
}

#[derive(Debug)]
struct Plant {
    associated_batch: u32,
    crop: String,
    start_date: String, //&Batch.start_date
    living: bool,
    stage: String, //germination, seedling, planted, havested
}

//I'll come back for this. Currently hardcoded as a string in plant and batch fields
#[derive(Debug)]
enum Stage{
    Germination,
    Seedling,
    Planted,
    Harvested,
}

#[derive(Debug)]
struct Error;

impl Batch {
    //Result? I wonder how this can fail with real users?
    fn new() -> Batch {
        Batch {
            batch_id: 001,
            start_date: calendar::datetime(),
            //change inventory to crop
            inventory: build_inventory(String::from("corn"), 7, String::from("germination"))
        }
    }
    fn isolate_crop(&mut self, crop: &str) -> Result<Plant, Error>{
        let item = self.inventory.get_mut(crop).expect("Nothing found under: {crop}");
        if item.isolated >= item.quantity {
            println!("Cannot complete: isolated plants would exceed total quantity");
            return Err(Error)
        }
        
        item.isolated += 1;
        Ok(Plant {
            associated_batch: self.batch_id,   
            crop: item.name.clone(),
            start_date: self.start_date.clone(),
            living: true,
            stage: item.stage.clone(),
        })

    }
    fn add_inventory_item(&mut self, item: InventoryItem) -> () {
        self.inventory.insert(item.name.clone(), item);
    }

    fn remove_iventory_item(&mut self, key: &str) -> () {
        match self.inventory.remove(key) {
            Some(value) => println!("{value:?} removed"),
            None => println! ("Error removing{key}: item not found"),
        }
    }
}

fn build_inventory(
    crop_name: String, 
    quantity: u32,  
    stage: String,
) -> HashMap <String, InventoryItem> {
    let crop_name = InventoryItem {
        name: crop_name.clone() ,
        quantity: quantity,
        stage: stage,
        isolated: 0,
    };
    let mut _hashmap = HashMap::new();

    _hashmap.insert(crop_name.name.clone(), crop_name);

    return _hashmap
    
}

fn main () {
    let mut test = Batch::new();
    let solo_plant = test.isolate_crop("corn");


    dbg!(test);
    dbg!(solo_plant);
}





mod helpers{
//Organizing helper functions 
    pub mod calendar {
    use chrono::prelude::*;
        pub fn datetime() -> String {
            let dt: DateTime<Local> = Local::now();

            dt.format("%m-%d-%Y, %H:%M%P").to_string()
        }

        pub fn date() -> String {
            let date: DateTime<Local> = Local::now();

            date.format("%m-%d-%Y").to_string()

        }
    }
}