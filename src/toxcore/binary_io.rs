/*
    Copyright © 2016 Zetok Zalbavar <zexavexxe@gmail.com>

    This file is part of Tox.

    Tox is libre software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Tox is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Tox.  If not, see <http://www.gnu.org/licenses/>.
*/

//! Functions for binary IO.


/// Serialization into bytes.
pub trait ToBytes {
    /// Serialize into bytes.
    fn to_bytes(&self) -> Vec<u8>;
}

/// De-serialize from bytes, or return `None` if de-serialization failed.
pub trait FromBytes<Output> {
    /// De-serialize from bytes, or return `None` if de-serialization failed.
    fn from_bytes(bytes: &[u8]) -> Option<Output>;
}


/// Safely cast `[u8; 2]` to `u16` using shift+or.
pub fn array_to_u16(array: &[u8; 2]) -> u16 {
    trace!("Casting array to u16 from array: {:?}", array);
    let mut result: u16 = 0;
    for byte in 0..array.len() {
        result = result << 8;
        result = result | array[1 - byte] as u16;
    }
    result
}

/// Safely cast `u16` to `[u8; 2]`.
pub fn u16_to_array(num: u16) -> [u8; 2] {
    trace!("Casting u16 to array from u16: {}", num);
    let mut array: [u8; 2] = [0; 2];
    for n in 0..array.len() {
        array[n] = (num >> (8 * n)) as u8;
    }
    array
}


/// Safely cast `&[u8; 4]` to `u32`.
pub fn array_to_u32(array: &[u8; 4]) -> u32 {
    trace!("Casting array to u32 from array: {:?}", array);
    let mut result: u32 = 0;
    for byte in 0..array.len() {
        result = result << 8;
        result = result | array[3 - byte] as u32;
    }
    result
}


/// Safely cast `&[u8; 8]` to `u64`.
pub fn array_to_u64(array: &[u8; 8]) -> u64 {
    trace!("Casting array to u64 from array: {:?}", array);
    let mut result: u64 = 0;
    for byte in 0..array.len() {
        result = result << 8;
        result = result | array[7 - byte] as u64;
    }
    result
}

/// Safely cast `u64` to `[u8; 8]`
pub fn u64_to_array(num: u64) -> [u8; 8] {
    trace!("Casting u64 to array from u64: {}", num);
    let mut array: [u8; 8] = [0; 8];
    for n in 0..array.len() {
        array[n] = (num >> (8 * n)) as u8;
    }
    array
}
