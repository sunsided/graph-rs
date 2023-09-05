//! A "Scotland Yard" game type map of London.

#![allow(dead_code)]

use crate::node_address::NodeAddress;
use crate::Graph;

#[derive(Debug)]
pub struct Station {
    id: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum ConnectionType {
    /// A subway line.
    Underground,
    /// A bus line
    Bus,
    /// A boat line.
    Ferry,
    /// A taxi line.
    Taxi,
}

pub fn london_graph() -> Graph<Station, ConnectionType> {
    let mut graph = Graph::default();

    let stations: Vec<_> = (1..=199)
        .into_iter()
        .map(|id| graph.add(Station { id }))
        .collect();

    connect_taxi(&mut graph, &stations);
    connect_bus(&mut graph, &stations);
    connect_subways(&mut graph, &stations);
    connect_ferries(&mut graph, &stations);

    graph
}

fn connect_taxi(mut graph: &mut Graph<Station, ConnectionType>, stations: &[NodeAddress]) {
    bidir(&mut graph, &stations, 1, 8, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 1, 9, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 2, 10, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 2, 20, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 3, 4, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 4, 13, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 5, 15, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 5, 16, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 6, 7, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 6, 29, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 7, 17, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 8, 18, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 8, 19, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 9, 19, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 9, 20, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 10, 11, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 10, 21, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 10, 34, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 11, 22, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 12, 23, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 13, 14, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 13, 23, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 13, 24, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 14, 15, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 14, 25, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 15, 16, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 15, 26, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 15, 28, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 16, 28, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 16, 29, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 17, 29, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 17, 30, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 17, 42, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 18, 31, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 18, 43, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 19, 32, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 20, 33, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 21, 33, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 22, 23, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 22, 34, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 22, 35, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 23, 37, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 24, 37, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 24, 38, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 25, 39, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 26, 27, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 26, 39, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 27, 40, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 28, 41, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 29, 41, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 29, 42, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 29, 55, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 30, 42, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 31, 43, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 31, 44, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 32, 33, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 32, 44, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 32, 45, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 33, 46, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 34, 47, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 34, 48, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 35, 36, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 35, 48, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 36, 37, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 36, 49, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 37, 50, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 38, 50, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 38, 51, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 39, 52, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 40, 41, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 41, 54, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 42, 56, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 42, 72, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 43, 57, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 44, 58, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 45, 46, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 45, 58, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 45, 59, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 45, 60, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 46, 47, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 46, 61, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 47, 62, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 48, 62, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 48, 63, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 49, 50, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 49, 66, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 50, 67, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 51, 52, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 51, 67, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 51, 68, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 52, 69, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 53, 54, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 53, 69, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 54, 55, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 54, 70, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 55, 71, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 56, 91, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 60, 76, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 61, 62, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 61, 76, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 61, 78, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 62, 79, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 63, 64, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 63, 79, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 64, 65, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 64, 81, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 65, 82, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 66, 67, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 66, 82, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 67, 68, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 67, 83, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 67, 84, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 68, 69, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 68, 85, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 69, 86, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 69, 89, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 70, 71, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 70, 87, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 71, 72, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 71, 89, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 72, 90, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 72, 91, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 73, 74, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 73, 92, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 74, 92, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 74, 94, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 75, 94, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 76, 77, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 77, 78, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 77, 95, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 77, 96, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 78, 79, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 78, 97, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 79, 98, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 80, 99, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 80, 100, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 81, 82, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 81, 100, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 82, 101, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 83, 101, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 83, 102, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 84, 85, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 85, 86, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 86, 87, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 86, 103, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 87, 88, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 87, 105, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 88, 117, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 89, 105, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 90, 91, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 90, 105, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 91, 105, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 91, 107, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 92, 93, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 93, 94, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 94, 95, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 95, 122, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 96, 97, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 96, 109, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 97, 98, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 97, 109, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 98, 99, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 98, 110, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 99, 110, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 99, 112, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 100, 101, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 100, 112, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 100, 113, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 101, 114, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 102, 103, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 102, 115, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 104, 116, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 105, 106, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 105, 108, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 106, 107, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 107, 119, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 108, 117, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 108, 119, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 108, 135, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 109, 124, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 110, 111, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 111, 112, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 111, 130, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 112, 125, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 113, 114, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 113, 125, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 114, 126, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 114, 131, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 114, 132, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 115, 126, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 115, 127, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 116, 117, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 116, 118, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 116, 127, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 117, 129, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 118, 129, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 118, 134, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 118, 142, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 119, 136, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 120, 121, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 120, 144, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 121, 122, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 121, 145, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 122, 123, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 122, 146, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 123, 124, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 123, 137, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 123, 149, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 124, 130, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 124, 138, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 125, 131, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 126, 127, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 127, 133, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 127, 134, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 128, 142, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 128, 143, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 128, 160, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 128, 172, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 128, 188, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 129, 135, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 129, 142, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 129, 143, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 130, 131, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 130, 139, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 132, 140, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 133, 141, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 134, 141, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 134, 142, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 135, 136, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 135, 143, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 135, 161, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 136, 161, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 136, 162, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 137, 147, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 138, 150, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 138, 152, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 139, 140, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 139, 153, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 139, 154, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 140, 154, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 140, 156, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 141, 142, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 141, 158, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 142, 158, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 143, 160, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 144, 145, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 144, 177, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 145, 146, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 146, 147, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 146, 163, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 147, 163, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 147, 164, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 148, 149, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 148, 164, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 149, 150, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 149, 165, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 150, 151, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 151, 152, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 151, 165, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 151, 166, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 152, 153, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 153, 154, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 153, 165, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 153, 166, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 153, 167, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 154, 155, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 155, 156, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 155, 167, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 155, 168, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 156, 157, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 156, 169, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 157, 158, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 157, 170, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 158, 159, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 158, 142, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 159, 170, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 159, 172, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 159, 186, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 159, 198, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 160, 161, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 160, 173, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 161, 174, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 162, 175, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 163, 164, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 163, 177, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 163, 178, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 164, 178, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 164, 179, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 165, 179, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 165, 180, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 166, 181, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 166, 183, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 167, 183, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 167, 168, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 169, 184, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 170, 185, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 171, 173, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 171, 175, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 171, 199, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 172, 187, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 173, 174, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 173, 188, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 174, 175, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 176, 177, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 176, 189, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 178, 189, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 178, 191, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 179, 191, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 180, 181, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 180, 192, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 180, 193, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 181, 182, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 181, 193, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 182, 195, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 183, 196, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 184, 185, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 184, 197, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 185, 186, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 185, 198, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 186, 198, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 187, 188, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 187, 198, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 188, 199, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 189, 190, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 190, 191, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 191, 192, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 192, 194, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 193, 194, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 194, 195, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 195, 197, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 196, 197, ConnectionType::Taxi);
    bidir(&mut graph, &stations, 198, 199, ConnectionType::Taxi);
}

fn connect_bus(mut graph: &mut Graph<Station, ConnectionType>, stations: &[NodeAddress]) {
    bidir(&mut graph, &stations, 1, 46, ConnectionType::Bus);
    bidir(&mut graph, &stations, 1, 58, ConnectionType::Bus);
    bidir(&mut graph, &stations, 3, 22, ConnectionType::Bus);
    bidir(&mut graph, &stations, 3, 23, ConnectionType::Bus);
    bidir(&mut graph, &stations, 7, 42, ConnectionType::Bus);
    bidir(&mut graph, &stations, 13, 14, ConnectionType::Bus);
    bidir(&mut graph, &stations, 13, 23, ConnectionType::Bus);
    bidir(&mut graph, &stations, 13, 52, ConnectionType::Bus);
    bidir(&mut graph, &stations, 14, 15, ConnectionType::Bus);
    bidir(&mut graph, &stations, 15, 29, ConnectionType::Bus);
    bidir(&mut graph, &stations, 15, 41, ConnectionType::Bus);
    bidir(&mut graph, &stations, 22, 23, ConnectionType::Bus);
    bidir(&mut graph, &stations, 22, 34, ConnectionType::Bus);
    bidir(&mut graph, &stations, 22, 56, ConnectionType::Bus);
    bidir(&mut graph, &stations, 23, 67, ConnectionType::Bus);
    bidir(&mut graph, &stations, 29, 41, ConnectionType::Bus);
    bidir(&mut graph, &stations, 29, 42, ConnectionType::Bus);
    bidir(&mut graph, &stations, 29, 55, ConnectionType::Bus);
    bidir(&mut graph, &stations, 34, 46, ConnectionType::Bus);
    bidir(&mut graph, &stations, 34, 58, ConnectionType::Bus);
    bidir(&mut graph, &stations, 34, 63, ConnectionType::Bus);
    bidir(&mut graph, &stations, 41, 52, ConnectionType::Bus);
    bidir(&mut graph, &stations, 41, 87, ConnectionType::Bus);
    bidir(&mut graph, &stations, 42, 72, ConnectionType::Bus);
    bidir(&mut graph, &stations, 46, 58, ConnectionType::Bus);
    bidir(&mut graph, &stations, 46, 78, ConnectionType::Bus);
    bidir(&mut graph, &stations, 52, 67, ConnectionType::Bus);
    bidir(&mut graph, &stations, 52, 86, ConnectionType::Bus);
    bidir(&mut graph, &stations, 55, 89, ConnectionType::Bus);
    bidir(&mut graph, &stations, 63, 65, ConnectionType::Bus);
    bidir(&mut graph, &stations, 63, 79, ConnectionType::Bus);
    bidir(&mut graph, &stations, 63, 100, ConnectionType::Bus);
    bidir(&mut graph, &stations, 65, 67, ConnectionType::Bus);
    bidir(&mut graph, &stations, 65, 82, ConnectionType::Bus);
    bidir(&mut graph, &stations, 67, 82, ConnectionType::Bus);
    bidir(&mut graph, &stations, 67, 86, ConnectionType::Bus);
    bidir(&mut graph, &stations, 67, 102, ConnectionType::Bus);
    bidir(&mut graph, &stations, 72, 105, ConnectionType::Bus);
    bidir(&mut graph, &stations, 72, 107, ConnectionType::Bus);
    bidir(&mut graph, &stations, 77, 78, ConnectionType::Bus);
    bidir(&mut graph, &stations, 77, 94, ConnectionType::Bus);
    bidir(&mut graph, &stations, 77, 124, ConnectionType::Bus);
    bidir(&mut graph, &stations, 78, 79, ConnectionType::Bus);
    bidir(&mut graph, &stations, 82, 100, ConnectionType::Bus);
    bidir(&mut graph, &stations, 82, 140, ConnectionType::Bus);
    bidir(&mut graph, &stations, 86, 87, ConnectionType::Bus);
    bidir(&mut graph, &stations, 86, 102, ConnectionType::Bus);
    bidir(&mut graph, &stations, 87, 105, ConnectionType::Bus);
    bidir(&mut graph, &stations, 89, 105, ConnectionType::Bus);
    bidir(&mut graph, &stations, 100, 111, ConnectionType::Bus);
    bidir(&mut graph, &stations, 102, 127, ConnectionType::Bus);
    bidir(&mut graph, &stations, 105, 107, ConnectionType::Bus);
    bidir(&mut graph, &stations, 105, 108, ConnectionType::Bus);
    bidir(&mut graph, &stations, 107, 161, ConnectionType::Bus);
    bidir(&mut graph, &stations, 108, 116, ConnectionType::Bus);
    bidir(&mut graph, &stations, 108, 135, ConnectionType::Bus);
    bidir(&mut graph, &stations, 116, 127, ConnectionType::Bus);
    bidir(&mut graph, &stations, 116, 142, ConnectionType::Bus);
    bidir(&mut graph, &stations, 122, 123, ConnectionType::Bus);
    bidir(&mut graph, &stations, 122, 144, ConnectionType::Bus);
    bidir(&mut graph, &stations, 123, 124, ConnectionType::Bus);
    bidir(&mut graph, &stations, 123, 165, ConnectionType::Bus);
    bidir(&mut graph, &stations, 124, 153, ConnectionType::Bus);
    bidir(&mut graph, &stations, 127, 133, ConnectionType::Bus);
    bidir(&mut graph, &stations, 128, 142, ConnectionType::Bus);
    bidir(&mut graph, &stations, 128, 161, ConnectionType::Bus);
    bidir(&mut graph, &stations, 128, 187, ConnectionType::Bus);
    bidir(&mut graph, &stations, 128, 199, ConnectionType::Bus);
    bidir(&mut graph, &stations, 133, 140, ConnectionType::Bus);
    bidir(&mut graph, &stations, 142, 157, ConnectionType::Bus);
    bidir(&mut graph, &stations, 144, 163, ConnectionType::Bus);
    bidir(&mut graph, &stations, 153, 154, ConnectionType::Bus);
    bidir(&mut graph, &stations, 153, 180, ConnectionType::Bus);
    bidir(&mut graph, &stations, 153, 184, ConnectionType::Bus);
    bidir(&mut graph, &stations, 154, 156, ConnectionType::Bus);
    bidir(&mut graph, &stations, 156, 157, ConnectionType::Bus);
    bidir(&mut graph, &stations, 156, 184, ConnectionType::Bus);
    bidir(&mut graph, &stations, 157, 185, ConnectionType::Bus);
    bidir(&mut graph, &stations, 161, 199, ConnectionType::Bus);
    bidir(&mut graph, &stations, 163, 191, ConnectionType::Bus);
    bidir(&mut graph, &stations, 163, 165, ConnectionType::Bus);
    bidir(&mut graph, &stations, 165, 180, ConnectionType::Bus);
    bidir(&mut graph, &stations, 165, 191, ConnectionType::Bus);
    bidir(&mut graph, &stations, 176, 190, ConnectionType::Bus);
    bidir(&mut graph, &stations, 180, 184, ConnectionType::Bus);
    bidir(&mut graph, &stations, 180, 190, ConnectionType::Bus);
    bidir(&mut graph, &stations, 184, 185, ConnectionType::Bus);
    bidir(&mut graph, &stations, 185, 187, ConnectionType::Bus);
    bidir(&mut graph, &stations, 190, 191, ConnectionType::Bus);
}

fn connect_subways(mut graph: &mut Graph<Station, ConnectionType>, stations: &[NodeAddress]) {
    bidir(&mut graph, &stations, 1, 46, ConnectionType::Underground);
    bidir(&mut graph, &stations, 13, 46, ConnectionType::Underground);
    bidir(&mut graph, &stations, 13, 67, ConnectionType::Underground);
    bidir(&mut graph, &stations, 13, 89, ConnectionType::Underground);
    bidir(&mut graph, &stations, 46, 74, ConnectionType::Underground);
    bidir(&mut graph, &stations, 46, 79, ConnectionType::Underground);
    bidir(&mut graph, &stations, 67, 79, ConnectionType::Underground);
    bidir(&mut graph, &stations, 67, 89, ConnectionType::Underground);
    bidir(&mut graph, &stations, 67, 111, ConnectionType::Underground);
    bidir(&mut graph, &stations, 79, 93, ConnectionType::Underground);
    bidir(&mut graph, &stations, 79, 111, ConnectionType::Underground);
    bidir(&mut graph, &stations, 89, 128, ConnectionType::Underground);
    bidir(&mut graph, &stations, 89, 140, ConnectionType::Underground);
    bidir(&mut graph, &stations, 111, 153, ConnectionType::Underground);
    bidir(&mut graph, &stations, 111, 163, ConnectionType::Underground);
    bidir(&mut graph, &stations, 128, 185, ConnectionType::Underground);
    bidir(&mut graph, &stations, 140, 153, ConnectionType::Underground);
    bidir(&mut graph, &stations, 140, 153, ConnectionType::Underground);
    bidir(&mut graph, &stations, 153, 185, ConnectionType::Underground);
}

fn connect_ferries(mut graph: &mut Graph<Station, ConnectionType>, stations: &[NodeAddress]) {
    bidir(&mut graph, &stations, 108, 115, ConnectionType::Ferry);
    bidir(&mut graph, &stations, 115, 157, ConnectionType::Ferry);
    bidir(&mut graph, &stations, 157, 194, ConnectionType::Ferry);
}

/// Bidirectionally link nodes.
fn bidir(
    graph: &mut Graph<Station, ConnectionType>,
    stations: &[NodeAddress],
    from: usize,
    to: usize,
    connection: ConnectionType,
) {
    let from = from - 1;
    let to = to - 1;
    // graph.link_to(&stations[from], &stations[to], connection);
    // graph.link_to(&stations[to], &stations[from], connection);
    graph.link_bidir(&stations[to], &stations[from], connection);
}
