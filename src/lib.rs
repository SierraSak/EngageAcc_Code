#![feature(lazy_cell, ptr_sub_ptr)]
use std::cmp::Ordering;

use unity::prelude::*;
use unity::engine::Sprite;

use engage::{
    stream::Stream,
    gameicon::GameIcon,
    gamedata::{
        accessory::AccessoryData, unit::{
            UnitAccessory,
            UnitAccessoryList
        }, Gamedata
    },
};

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

#[unity::hook("App", "UnitAccessoryList", "get_Count")]
pub fn unitaccessorylist_get_count(_this: &mut UnitAccessoryList, _method_info: OptionalMethod) -> i32 {
    return 15;
}

#[unity::hook("App", "AccessoryData", "OnBuild")]
pub fn accessorydata_on_build_hook(this: &mut AccessoryData, method_info: OptionalMethod) {
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

#[unity::hook("App", "UnitAccessoryList", "CopyFrom")]
pub fn unitaccessorylist_copyfrom_hook(this: &mut UnitAccessoryList, list: &mut UnitAccessoryList, _method_info: OptionalMethod) {
    this.unit_accessory_array
        .iter_mut()
        .zip(list.unit_accessory_array.iter_mut())
        .for_each(|(dest, src)| {
            dest.index = src.index;
        });
}

#[unity::hook("App", "UnitAccessoryList", "Clear")]
pub fn unitaccessorylist_clear_hook(this: &mut UnitAccessoryList, _method_info: OptionalMethod) {
    this.unit_accessory_array.iter_mut().for_each(|acc| acc.index = 0);
}

#[skyline::hook(offset = 0x1f620a0)]
pub fn unitaccessorylist_add_hook(this: &mut UnitAccessoryList, accessory: Option<&mut AccessoryData>, index: usize, _method_info: OptionalMethod) -> bool {
    // Ray: I have no clue why this is done so I hope you do.

    let accessories = AccessoryData::get_list().expect("Couldn't reach AccessoryData List");

    if let Some(accessory) = accessory {
        for curr_acc in this.unit_accessory_array.iter_mut() { // Go through every entry in the array.
            // Grab the AccessoryData at that index in the XML
            if let Some(found) = accessories.get(curr_acc.index as usize) {
                // If an entry was found, check if the mask is similar and set the index to 0 if it is
                if accessory.mask == found.mask {
                    curr_acc.index = 0;
                }
            }
        }

        // Checks if index is within the array's length
        if index > this.unit_accessory_array.len() {
            // If index is beyond the array's length, this is a new accessory.
            for item in this.unit_accessory_array.iter_mut() {
                // If the index for the current item is 0
                if item.index == 0 {
                    // Set it to the index of the accessory we received
                    item.index = accessory.parent.index;
                }
            }
        } else {
            // We can safely index in the array here because we already confirmed that we are within the acceptable indices for the array... I hope
            this.unit_accessory_array[index].index = accessory.parent.index;
        }

        true
    } else {
        false
    }
}

#[unity::hook("App", "UnitAccessoryList", "IsExist")]
pub fn unitaccessorylist_is_exist_hook(this: &mut UnitAccessoryList, accessory: Option<&mut AccessoryData>, _method_info: OptionalMethod) -> bool {
    let accessories = AccessoryData::get_list().expect("Couldn't reach AccessoryData List");

    // This is your old "if accessory == 0x0 {}". In the context of talking with C, Rust allows you to use Option<> on a pointer to signify that it could be null.
    // That gives you plenty of fancy ways to check for null
    accessory.is_some_and(|accessory| {
        // Looks for the AID of the provided accessory in the XML and return the index of the matching entry
        this.unit_accessory_array
            .iter() // Go through every entry in the array.
            .any(|curr_acc| { // Confirms if any of the items in the array fulfills the condition.
                // Grab the AccessoryData at that index in the XML if it's present, and if it is, compare the AIDs.
                // Return false if the index is out of bounds OR the AIDs don't match
                accessories.get(curr_acc.index as usize).is_some_and(|item| {
                    item.aid.get_string().unwrap() == accessory.aid.get_string().unwrap()
                })
            })
    })
}

#[unity::hook("App", "UnitAccessoryList", "Serialize")]
pub fn unitaccessorylist_serialize_hook(this: &mut UnitAccessoryList, stream: &mut Stream, _method_info: OptionalMethod) {
    stream.write_int(1).expect("Could not write version number when serializing UnitAccessoryList");

    // TODO: Simplify by calling serialize on the UnitAccessoryList directly
    this.unit_accessory_array
        .iter_mut()
        .for_each(|curr_acc| {
            curr_acc.serialize(stream);
        });
}

#[unity::hook("App", "UnitAccessoryList", "Deserialize")]
pub fn unitaccessorylist_deserialize_hook(this: &mut UnitAccessoryList, stream: &mut Stream, _method_info: OptionalMethod) {
    this.unit_accessory_array
            .iter_mut()
            .for_each(|curr_acc| {
                curr_acc.index = 0;
            });

    let version_check = stream.read_int().expect("Could not read the version from the UnitAccessoryList block in the savefile");

    if version_check > 0 {
        // Deserializes as many items as there are in the array
        this.unit_accessory_array.iter_mut()
            .for_each(|curr_acc| {
                curr_acc.deserialize(stream);
            });
    } else {
        // Just deserializes the 4 original items
        this.unit_accessory_array[..4].iter_mut()
            .for_each(|curr_acc| {
                curr_acc.deserialize(stream);
            });
    }
}

#[unity::hook("App", "GameIcon", "TryGetAccessoryKinds")]
pub fn gameicon_try_get_accessory_kinds_hook(accessory_kinds: i32, _method_info: OptionalMethod) -> &'static Sprite
{
    let mut i = "Face";
    //Tilde's Custom icon code doesn't currently support System sprites, so I have temporarily commented the custom ones out for the sake of
    //looking nice ingame.
    match accessory_kinds {
        0 => i = "Clothes",
        //1 => i = "Head",
        //2 => i = "Face",
        //3 => i = "Hand",
        //5 => i = "Back",
        //6 => i = "Dye",
        //7 => i = "Style",
        _=> i = "Face",
    }
    let spriteim = GameIcon::try_get_system(i).expect("Couldn't get sprite for AccessoryKind");

    return spriteim;
}

#[unity::hook("App", "UnitAccessoryList", ".ctor")]
pub fn unitaccessorylist_ctor_hook(this: &mut UnitAccessoryList, method_info: OptionalMethod,)
{
    call_original!(this, method_info);

    // Il2CppArray can be turned into a slice (https://doc.rust-lang.org/std/primitive.slice.html) and slices can be iterated (https://doc.rust-lang.org/std/iter/trait.Iterator.html) on, so we can just walk through every item in the array and manipulate them
    // println!("Array length: {}", this.unit_accessory_array.len());

    this.unit_accessory_array
        .iter_mut()
        .for_each(|item| {
            *item = UnitAccessory::instantiate()
                .map(|acc| {
                    acc.index = 0 as i32;
                    acc
                })
                .unwrap();
        });
}

#[skyline::main(name = "TestProject")]
pub fn main() {
    // Install a panic handler for your plugin, allowing you to customize what to do if there's an issue in your code.
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    
    
    skyline::install_hooks!(
        accessorydata_on_build_hook,
        gameicon_try_get_accessory_kinds_hook,
        unitaccessorylist_ctor_hook,
        unitaccessorylist_serialize_hook,
        unitaccessorylist_deserialize_hook,
        unitaccessorylist_copyfrom_hook,
        unitaccessorylist_get_count,
        unitaccessorylist_clear_hook,
        unitaccessorylist_is_exist_hook,
        unitaccessorylist_add_hook
    );

    skyline::patching::Patch::in_text(0x01f61c00).bytes(&[0x01, 0x02, 0x80, 0x52]).expect("Couldn’t patch that shit for some reasons");
    skyline::patching::Patch::in_text(0x027b5d70).bytes(&[0xDF, 0x3E, 0x00, 0x71]).expect("Couldn’t patch that shit for some reasons");
    skyline::patching::Patch::in_text(0x027b5d8c).bytes(&[0xDF, 0x42, 0x00, 0x71]).expect("Couldn’t patch that shit for some reasons");

    //Patch Get_Count
    //skyline::patching::Patch::in_text(0x01f61b10).bytes(&[0xE0, 0x01, 0x80, 0x52]).expect("Couldn’t patch that shit for some reasons");

    //skyline::patching::Patch::in_text(0x027bffcc).bytes(&[0x1F, 0x20, 0x03, 0xD5]).expect("Couldn’t patch that shit for some reasons");
}
