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


use crate::sections::{MultiSection};
use wasmtime::component::{bindgen, ResourceTable, Resource};
use std::collections::hash_map::Entry;

pub type SectionStr = MultiSection<String>;

bindgen!({
    world :"dyparser",
    with : {
        "loara:dyparser/types/section" : SectionStr,
    }
});

use crate::plugins::loara::dyparser;

pub struct Plugin{
    comp : wasmtime::component::Component,
}

struct PluginState<I>{
    table : ResourceTable,
    file : I,
}

struct PluginRuntime<I>{
    store : wasmtime::Store<PluginState<I>>,
    plg : Dyparser,
}

impl<I> dyparser::types::Host for PluginState<I> where I : Iterator<Item = char>{
    fn next(&mut self) -> Option<char> {
        self.file.next()
    }
}

impl<I> dyparser::types::HostSection for PluginState<I>{
    fn new(&mut self) -> Resource<SectionStr> {
        self.table.push(SectionStr::default()).unwrap()
    }
    fn add_field(&mut self, sec : Resource<SectionStr>, key : String, val : String) {
        let secobj = self.table.get_mut(&sec).unwrap();
        match secobj.keys.entry(key) {
            Entry::Occupied(mut oc) => {
                oc.get_mut().push(val);
            }
            Entry::Vacant(va) => {
                va.insert(vec![val]);
            }
        }
    }
    fn add_section(&mut self, sec : Resource<SectionStr>, key : String, val : Resource<SectionStr>) {
        let valobj = self.table.delete(val).unwrap();
        let secobj = self.table.get_mut(&sec).unwrap();
        match secobj.sec.entry(key) {
            Entry::Occupied(mut oc) => {
                oc.get_mut().push(valobj);
            }
            Entry::Vacant(va) => {
                va.insert(vec![valobj]);
            }
        }
    }
    fn drop(&mut self, sec : Resource<SectionStr>) -> wasmtime::Result<()> {
        let _t = self.table.delete(sec)?;
        Ok(())
    }
}

impl Plugin{
    fn load_from_file<P : AsRef<std::path::Path>>(file : P) -> Self {
        let eng = wasmtime::Engine::default();

        Self{
            comp : wasmtime::component::Component::from_file(&eng, file).unwrap(),
        }
    }
    fn load_plugin(name : &str) -> Self {
        let pdir = directories::ProjectDirs::from("org", "loara", "dyparser").unwrap();
        let mut pathbuf = std::path::PathBuf::from(pdir.data_dir());
        pathbuf.push("plugins");
        pathbuf.push(name);
        pathbuf.set_extension("wasm");
        Self::load_from_file(pathbuf)
    }
    pub fn load_default() -> Self {
        Self::load_plugin("default")
    }

    fn instantiate<I>(&mut self, data : I) -> PluginRuntime<I> where I : Iterator<Item = char> {
        let mut link = wasmtime::component::Linker::new(self.comp.engine());
        Dyparser::add_to_linker(&mut link, |st : &mut PluginState<I>| st).unwrap();
        let mut st = wasmtime::Store::new(self.comp.engine(), PluginState{
            table : ResourceTable::new(),
            file : data,
        });

        let par = Dyparser::instantiate(&mut st, &self.comp, &link).unwrap();

        PluginRuntime{
            store : st,
            plg : par,
        }
    }

    pub fn parse_config<P : AsRef<std::path::Path>>(&mut self, filename : P) -> SectionStr {
        let file = std::fs::read_to_string(filename).unwrap();
        let ret = self.instantiate(file.chars()).parse();
        ret
    }

}

impl<I> PluginRuntime<I> {
    fn parse(&mut self) -> SectionStr {
        let res = self.plg.loara_dyparser_parser().call_parse_stream(&mut self.store).unwrap();
        self.store.data_mut().table.delete(res).unwrap()
    }
}

