//    dyconfig Configuration file parser through Wasm-WASI plugins
//    Copyright (C) 2024  Paolo De Donato
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.


use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct MultiSection<T>{
    pub(crate) keys : HashMap<String, Vec<T>>,
    pub(crate) sec : HashMap<String, Vec<MultiSection<T>>>,
}

impl<T> MultiSection<T>{
    pub fn get_kv(&self) -> &HashMap<String, Vec<T>> {
        &self.keys
    }
    pub fn get_subsections(&self) -> &HashMap<String, Vec<MultiSection<T>>> {
        &self.sec
    }
    pub fn map<U, F : FnMut(T) -> U + Clone>(self, f : F) -> MultiSection<U> {
        let mkey = self.keys.into_iter().map(
                |(k, v)| (k, v.into_iter().map(f.clone()).collect())
            ).collect();
        let msec = self.sec.into_iter().map(
                |(k, v)| (k, v.into_iter().map(
                        |isec| isec.map(f.clone())
                    ).collect())
            ).collect();
        MultiSection{
            keys : mkey,
            sec : msec,
        }
    }
}

