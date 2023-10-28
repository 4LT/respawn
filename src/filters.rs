use std::ffi::{CString, CStr};
use std::cell::RefCell;
use quake_util::qmap::{QuakeMap, Entity, Edict};

const NOT_EASY_FLAG: i32 = 1 << 8;
const NOT_NORMAL_FLAG: i32 = 1 << 9;
const NOT_HARD_FLAG: i32 = 1 << 10;

const SKILL_MASK: i32 = NOT_EASY_FLAG | NOT_NORMAL_FLAG | NOT_HARD_FLAG;

const EASY_ONLY_FLAGS: i32 = NOT_NORMAL_FLAG | NOT_HARD_FLAG;
const NORMAL_ONLY_FLAGS: i32 = NOT_EASY_FLAG | NOT_HARD_FLAG;
const HARD_ONLY_FLAGS: i32 = NOT_EASY_FLAG | NOT_NORMAL_FLAG;

pub fn patch_skill(map: &mut QuakeMap) {
    let mut new_ents = Vec::new();

    for ent in &mut map.entities {
        let classname_key = CString::new("classname").unwrap();
        let worldspawn = CString::new("worldspawn").unwrap();

        if ent.edict.get(&classname_key) != Some(&worldspawn) {
            new_ents.append(&mut patch_skill_entity(ent));
        }
    }

    map.entities.append(&mut new_ents);
}

fn patch_skill_entity(ent: &mut Entity) -> Vec<Entity> {
    let skills = [
        (
            &b"easy:"[..],
            RefCell::new(Edict::new()),
            EASY_ONLY_FLAGS,
        ),
        (
            &b"normal:"[..],
            RefCell::new(Edict::new()),
            NORMAL_ONLY_FLAGS,
        ),
        (
            &b"hard:"[..],
            RefCell::new(Edict::new()),
            HARD_ONLY_FLAGS,
        ),
    ];

    let mut prefixed_keys: Vec<CString> = Vec::new();

    for (key, value) in &ent.edict {
        let key_bytes = key.to_bytes_with_nul();

        for (prefix, edict_cell, _) in &skills {
            if key_bytes.starts_with(prefix) {
                prefixed_keys.push(key.clone());

                let patch_key = (&key_bytes[prefix.len()..]).to_vec();
                let patch_key = unsafe {
                    CString::from_vec_with_nul_unchecked(patch_key)
                };

                edict_cell.borrow_mut().insert(
                    patch_key,
                    value.clone(),
                );
            }
        }
    }

    for key in prefixed_keys {
        ent.edict.remove(&key);
    }

    skills.into_iter()
        .filter_map(|(_, edict_cell, flags)| {
            let mut patch_edict = edict_cell.take();

            if patch_edict.is_empty() {
                None
            } else {
                for (key, value) in &ent.edict {
                    if !patch_edict.contains_key(key) {
                        patch_edict.insert(key.clone(), value.clone());
                    }
                }

                let spawnflags_key = CString::new("spawnflags").unwrap();

                let mut spawnflags = match ent.edict.get(&spawnflags_key) {
                    None => 0i32,
                    Some(flags) => parse_int(&flags),
                };

                spawnflags = (spawnflags & !SKILL_MASK) | flags;
                let spawnflags = CString::new(spawnflags.to_string()).unwrap();

                patch_edict.insert(spawnflags_key, spawnflags);

                Some(Entity {
                    edict: patch_edict,
                    brushes: ent.brushes.clone()
                })
            }
        }).collect()
}

fn parse_int(string: &CStr) -> i32 {
    let mut value = 0i32;
    let mut bytes = string.to_bytes().iter().copied().peekable();

    while match bytes.peek() {
        None => false,
        Some(b) => b.is_ascii_whitespace(),
    } {
        bytes.next();
    }

    loop {
        let digit = match bytes.next() {
            Some(b) if b >= b'0' && b <= b'9' =>  b - b'0',
            _ => {
                break;
            },
        };

        value*= 10;
        value+= i32::from(digit);
    }

    value
}
