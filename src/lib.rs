#![feature(lazy_cell, ptr_sub_ptr)]

use unity::prelude::*;
use engage::gamedata::{StructDataGeneric, StructData, StructBaseFields, StructDataStaticFields};

#[unity::class("App", "AccessoryData")]
pub struct AccessoryData {
    pub parent: StructBaseFields,
    pub aid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub name_m: &'static Il2CppString,
    pub help_m: &'static Il2CppString,
    pub name_f: &'static Il2CppString,
    pub help_f: &'static Il2CppString,
    pub first: bool,
    pub amiibo: bool,
    pub condition_cid: &'static Il2CppString,
    pub condition_gender: i32,
    pub condition_skills: &'static [Il2CppString; 0],
    pub gid: &'static Il2CppString,
    pub asset: &'static Il2CppString,
    pub price: i32,
    pub iron: i32,
    pub steel: i32,
    pub silver: i32,
    pub mask: i32,
    pub kind: i32,
    pub god_data: u64,
    pub flag_name: &'static Il2CppString,
    
    // ...
}

// Due to how you use the enum, let's not use these for now
#[repr(i32)]
pub enum AccessoryDataMasks {
    Body = 1,
    Head = 2,
    Face = 4,
    Back = 8,
    // Expand here
}

// Due to how you use the enum, let's not use these for now
#[repr(i32)]
pub enum AccessoryDataKinds {
    Body = 0,
    Head = 1,
    Face = 2,
    Back = 3,
}

#[unity::class("App", "UnitAccessoryList")]
pub struct UnitAccessoryList {
    pub unit_accessory_array: &'static mut Il2CppArray<&'static mut UnitAccessory>,
}

#[unity::class("App", "UnitAccessory")]
pub struct UnitAccessory {
    pub index: i32,
}


#[unity::hook("App", "UnitAccessoryList", "get_count")]
pub fn app_unitaccessorylist_getcount(_this: &mut UnitAccessoryList, _method_info: OptionalMethod) -> i32 {
    return 15;
}


#[unity::hook("App", "AccessoryData", "OnBuild")]
pub fn onbuild_accessory_data_hook(this: &mut AccessoryData, method_info: OptionalMethod) {
    call_original!(this, method_info);

    if this.mask > 8
    {
        match this.mask{
            16 => this.kind = 4,
            32 => this.kind = 5,
            64 => this.kind = 6,
            128 => this.kind = 7,
            256 => this.kind = 8,
            512 => this.kind = 9,
            1024 => this.kind = 10,
            2048 => this.kind = 11,
            4096 => this.kind = 12,
            8192 => this.kind = 13,
            16384 => this.kind = 14,
            32768 => this.kind = 15,
            _=> this.kind = 1,
        }
    }
}

//Currently not compiling
//cannot borrow data in dereference of `Array<&unity::prelude::Il2CppObject<unit_accessory>>` as mutable

// You did not specify in UnitAccessoryList that the content of the array can be mut(ated), so Rust stopped you
#[unity::hook("App", "UnitAccessoryList", "Clear")]
pub fn clear_unitaccessorylist_hook(this: &mut UnitAccessoryList, _method_info: OptionalMethod,)
{
   //call_original!(this, method_info);

    // OLD
    //    let mut i = 0;
    //    while i < this.unit_accessory_array.len()
    //    {
    //        this.unit_accessory_array[i].index = 0;
    //        i += 1;
    //    }

    // NEW
    this.unit_accessory_array.iter_mut().for_each(|acc| acc.index = 0);
}

#[skyline::hook(offset = 0x1F62090)]
pub fn add_unitaccessorylist_hook(this: &mut UnitAccessoryList, accessory: &mut AccessoryData, index: usize, _method_info: OptionalMethod,)
{
    // OLD

    // let mut i = 0;
    // let mut equipped_acc_index = 0;
    // let mut acc_check = 0;

    // while i < this.unit_accessory_array.len()
    // {
    //     equipped_acc_index = this.unit_accessory_array[i].index;
    //     acc_check = TryGet Accessory XML data from acc_index
    //     if acc_check != 0 and accessory.mask == acc_check.mask
    //     {
    //         this.unit_accessory_array[i].index = 0;
    //     }
    // }

    // NEW

    let structdata: &Il2CppClass = get_generic_class!(StructDataGeneric<AccessoryData>).unwrap();
    let accessory_table = Il2CppObject::<StructData>::from_class(structdata).unwrap();
    let accessories = accessory_table.get_class().get_static_fields::<StructDataStaticFields<AccessoryData>>();

    this.unit_accessory_array
        .iter_mut() 
        .for_each(|curr_acc| { // Go through every entry in the array.
            // Grab the AccessoryData at that index in the XML
            if let Some (found) = accessories.s_list.list.items.get(curr_acc.index as usize) {
                // If an entry was found, check if the mask is similar and set the index to 0 if it is
                if accessory.mask == found.mask {
                    curr_acc.index = 0;
                }
            }
        });

    // OLD


    // i = 0;

    // if index < 0
    // {
    //     equipped_acc_index = this.unit_accessory_array[i].index;
    //     if equipped_acc_index == 0
    //     {
    //         this.unit_accessory_array[i].index = accessory.super.super.super.Index;
    //     }
    //     i += 1;
    // }
    // else if index < this.unit_accessory_array.len()
    // {
    //     this.unit_accessory_array[index].index = accessory.super.super.super.Index;
    //     i += 1;
    // }

    // NEW

    // Checks if index is within 0 and the array's len
    if (0..this.unit_accessory_array.len()).contains(&index) {
        // We can safely index in the array here because we already confirmed that we are within the acceptable indices for the array... I hope
        this.unit_accessory_array[index].index = accessory.parent.index;
    } else {
        // If 0 is less than 0 or beyond the array's length
        this.unit_accessory_array
            .iter_mut()
            .for_each(|item| {
                // If the index for the current item is 0
                if item.index == 0 {
                    // Set it to the index of the accessory we received
                    item.index = accessory.parent.index;
                }
            });
    }
}

#[unity::hook("App", "UnitAccessoryList", "IsExist")]
pub fn unitaccessorylist_is_exist_hook(this: &mut UnitAccessoryList, accessory: Option<&mut AccessoryData>, _method_info: OptionalMethod) -> bool
{
    // This is your old "if accessory == 0x0 {}". In the context of talking with C, Rust allows you to use Option<> on a pointer to signify that it could be null.
    // That gives you plenty of fancy ways to check for null
    accessory.is_some_and(|accessory| {
        let structdata: &Il2CppClass = get_generic_class!(StructDataGeneric<AccessoryData>).unwrap();
        let accessory_table = Il2CppObject::<StructData>::from_class(structdata).unwrap();
        let accessories = accessory_table.get_class().get_static_fields::<StructDataStaticFields<AccessoryData>>();

        // Looks for the AID of the provided accessory in the XML and return the index of the matching entry
        this.unit_accessory_array
            .iter() // Go through every entry in the array.
            .any(|curr_acc| { // Confirms if any of the items in the array fulfills the condition.
                // Grab the AccessoryData at that index in the XML if it's present, and if it is, compare the AIDs.
                // Return false if the index is out of bounds OR the AIDs don't match
                accessories.s_list.list.items.get(curr_acc.index as usize).is_some_and(|item| {
                    item.aid.get_string().unwrap() == accessory.aid.get_string().unwrap()
                })
            })
    })  
}

#[unity::hook("App", "UnitAccessoryList", ".ctor")]
pub fn unitaccessorylist_ctor_hook(this: &mut UnitAccessoryList, method_info: OptionalMethod,)
{
    call_original!(this, method_info);
    
    // Il2CppArray can be turned into a slice (https://doc.rust-lang.org/std/primitive.slice.html) and slices can be iterated (https://doc.rust-lang.org/std/iter/trait.Iterator.html) on, so we can just walk through every item in the array and manipulate them
    this.unit_accessory_array.iter_mut().for_each(|entry| {
        *entry = UnitAccessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    });
}

#[unity::hook("App", "AccessoryShopChangeRoot", "OnSelectMenuItem")]
pub fn onselectmenuitem_accessory_data_hook(this: &(), accessory_data: &mut AccessoryData, method_info: OptionalMethod)
{
    call_original!(this, accessory_data, method_info)
}

#[skyline::main(name = "TestProject")]
pub fn main() {
    skyline::install_hooks!(unitaccessorylist_ctor_hook, onbuild_accessory_data_hook, app_unitaccessorylist_getcount, clear_unitaccessorylist_hook, unitaccessorylist_is_exist_hook, add_unitaccessorylist_hook);
    skyline::patching::Patch::in_text(0x01f61c00).bytes(&[0x01, 0x02, 0x80, 0x52]).expect("Couldn’t patch that shit for some reasons");
    skyline::patching::Patch::in_text(0x027b5d70).bytes(&[0xDF, 0x3E, 0x00, 0x71]).expect("Couldn’t patch that shit for some reasons");
    skyline::patching::Patch::in_text(0x027b5d8c).bytes(&[0xDF, 0x42, 0x00, 0x71]).expect("Couldn’t patch that shit for some reasons");
    //skyline::patching::Patch::in_text(0x027bffcc).bytes(&[0x1F, 0x20, 0x03, 0xD5]).expect("Couldn’t patch that shit for some reasons");
}
