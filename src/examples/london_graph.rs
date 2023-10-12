//! A "Scotland Yard" game type map of London.

#![allow(dead_code)]

use crate::astar::Heuristic;
use crate::node_address::NodeAddress;
use crate::Graph;

#[derive(Debug)]
pub struct Station {
    /// The station ID
    id: usize,
    /// The X coordinate of the station on the map, in 0..=768
    x: usize,
    /// The Y coordinate of the station on the map, in 0..=512
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

/// Heuristic for the London Graph
#[derive(Default, Copy, Clone)]
pub struct LondonGraphLocationHeuristic;

impl Heuristic<Station> for LondonGraphLocationHeuristic {
    fn heuristic(&self, from: &Station, to: &Station) -> f32 {
        let x = from.x as f32 - to.x as f32;
        let y = from.y as f32 - to.y as f32;
        x * x + y * y
    }
}

pub fn london_graph() -> Graph<Station, ConnectionType> {
    let mut graph = Graph::default();

    let stations = stations(&mut graph);

    connect_taxi(&mut graph, &stations);
    connect_bus(&mut graph, &stations);
    connect_subways(&mut graph, &stations);
    connect_ferries(&mut graph, &stations);

    graph
}

fn stations(graph: &mut Graph<Station, ConnectionType>) -> Vec<NodeAddress> {
    #[rustfmt::skip]
    let stations = [
        Station { id: 1, x: 110, y: 30 },
        Station { id: 2, x: 250, y: 35 },
        Station { id: 3, x: 302, y: 27 },
        Station { id: 4, x: 375, y: 30 },
        Station { id: 5, x: 585, y: 33 },
        Station { id: 6, x: 630, y: 35 },
        Station { id: 7, x: 702, y: 37 },
        Station { id: 8, x: 72, y: 58 },
        Station { id: 9, x: 127, y: 60 },

        Station { id: 10, x: 262, y: 70 },
        Station { id: 11, x: 302, y: 62 },
        Station { id: 12, x: 340, y: 65 },
        Station { id: 13, x: 405, y: 67 },
        Station { id: 14, x: 467, y: 47 },
        Station { id: 15, x: 525, y: 35 },
        Station { id: 16, x: 596, y: 67 },
        Station { id: 17, x: 700, y: 85 },
        Station { id: 18, x: 32, y: 90 },
        Station { id: 19, x: 100, y: 87 },

        Station { id: 20, x: 157, y: 85 },
        Station { id: 21, x: 219, y: 110 },
        Station { id: 22, x: 307, y: 105},
        Station { id: 23, x: 352, y: 92 },
        Station { id: 24, x: 426, y: 95 },
        Station { id: 25, x: 475, y: 80 },
        Station { id: 26, x: 512, y: 70 },
        Station { id: 27, x: 525, y: 95 },
        Station { id: 28, x: 560, y: 85 },
        Station { id: 29, x: 640, y: 105 },

        Station { id: 30, x: 725, y: 115 },
        Station { id: 31, x: 65, y: 122 },
        Station { id: 32, x: 137, y: 125 },
        Station { id: 33, x: 185, y: 120 },
        Station { id: 34, x: 279, y: 120 },
        Station { id: 35, x: 320, y: 140 },
        Station { id: 36, x: 350, y: 145 },
        Station { id: 37, x: 385, y: 125 },
        Station { id: 38, x: 424, y: 122 },
        Station { id: 39, x: 490, y: 107 },

        Station { id: 40, x: 540, y: 127 },
        Station { id: 41, x: 585, y: 117 },
        Station { id: 42, x: 687, y: 130 },
        Station { id: 43, x: 34, y: 135 },
        Station { id: 44, x: 92, y: 145 },
        Station { id: 45, x: 160, y: 155 },
        Station { id: 46, x: 210, y: 150 },
        Station { id: 47, x: 245, y: 132 },
        Station { id: 48, x: 285, y: 157 },
        Station { id: 49, x: 365, y: 167 },

        Station { id: 50, x: 395, y: 150 },
        Station { id: 51, x: 465, y: 150 },
        Station { id: 52, x: 597, y: 135 },
        Station { id: 53, x: 550, y: 165 },
        Station { id: 54, x: 575, y: 150 },
        Station { id: 55, x: 630, y: 145 },
        Station { id: 56, x: 720, y: 172 },
        Station { id: 57, x: 47, y: 167 },
        Station { id: 58, x: 112, y: 167 },
        Station { id: 59, x: 150, y: 200 },

        Station { id: 60, x: 185, y: 185 },
        Station { id: 61, x: 220, y: 192 },
        Station { id: 62, x: 255, y: 172 },
        Station { id: 63, x: 295, y: 210 },
        Station { id: 64, x: 327 , y: 210 },
        Station { id: 65, x: 355, y: 192 },
        Station { id: 66, x: 387, y: 195 },
        Station { id: 67, x: 420, y: 177 },
        Station { id: 68, x: 477, y: 175 },
        Station { id: 69, x: 520 , y: 185 },

        Station { id: 70, x: 577, y: 180 },
        Station { id: 71, x: 622, y: 175 },
        Station { id: 72, x: 675, y: 177 },
        Station { id: 73, x: 55 , y: 195 },
        Station { id: 74, x: 80, y: 215 },
        Station { id: 75, x: 120, y: 220 },
        Station { id: 76, x: 175, y: 217 },
        Station { id: 77, x: 192, y: 240 },
        Station { id: 78, x: 225 , y: 227 },
        Station { id: 79, x: 260, y: 215 },

        Station { id: 80, x: 327, y: 247 },
        Station { id: 81, x: 360, y: 247 },
        Station { id: 82, x: 385, y: 230 },
        Station { id: 83, x: 420, y: 240 },
        Station { id: 84, x: 460, y: 202 },
        Station { id: 85, x: 495, y: 205 },
        Station { id: 86, x: 532, y: 217 },
        Station { id: 87, x: 575, y: 235 },
        Station { id: 88, x: 602, y: 255 },
        Station { id: 89, x: 610 , y: 215 },

        Station { id: 90, x: 660, y: 200  },
        Station { id: 91, x: 705, y: 207 },
        Station { id: 92, x: 42, y: 230 },
        Station { id: 93, x: 45, y: 265 },
        Station { id: 94, x: 82, y: 265 },
        Station { id: 95, x: 137, y: 252 },
        Station { id: 96, x: 215, y: 275 },
        Station { id: 97, x: 245, y: 265 },
        Station { id: 98, x: 275, y: 255 },
        Station { id: 99, x: 302, y: 265 },

        Station { id: 100, x: 352, y: 280 },
        Station { id: 101, x: 387, y: 265 },
        Station { id: 102, x: 455, y: 237 },
        Station { id: 103, x: 500, y: 225 },
        Station { id: 104, x: 545, y: 260 },
        Station { id: 105, x: 645, y: 245 },
        Station { id: 106, x: 685, y: 240 },
        Station { id: 107, x: 725, y: 240 },
        Station { id: 108, x: 642, y: 310 },
        Station { id: 109, x: 250, y: 305 },

        Station { id: 110, x: 280, y: 380 },
        Station { id: 111, x: 290, y: 310 },
        Station { id: 112, x: 320, y: 292 },
        Station { id: 113, x: 360, y: 305 },
        Station { id: 114, x: 395, y: 305 },
        Station { id: 115, x: 460, y: 280 },
        Station { id: 116, x: 545, y: 310 },
        Station { id: 117, x: 597, y: 315 },
        Station { id: 118, x: 555, y: 345 },
        Station { id: 119, x: 712, y: 330 },

        Station { id: 120, x: 27, y: 355 },
        Station { id: 121, x: 65, y: 355 },
        Station { id: 122, x: 100, y: 250 },
        Station { id: 123, x: 185, y: 350 },
        Station { id: 124, x: 242, y: 335 },
        Station { id: 125, x: 330, y: 315 },
        Station { id: 126, x: 430, y: 315 },
        Station { id: 127, x: 492, y: 330 },
        Station { id: 128, x: 582, y: 442 },
        Station { id: 129, x: 597, y: 350 },

        Station { id: 130, x: 310, y: 345 },
        Station { id: 131, x: 350, y: 335 },
        Station { id: 132, x: 390, y: 330 },
        Station { id: 133, x: 465, y: 370 },
        Station { id: 134, x: 515, y: 360 },
        Station { id: 135, x: 635, y: 375 },
        Station { id: 136, x: 680, y: 390 },
        Station { id: 137, x: 160, y: 372 },
        Station { id: 138, x: 255, y: 360 },
        Station { id: 139, x: 320, y: 367 },

        Station { id: 140, x: 385, y: 365 },
        Station { id: 141, x: 487, y: 395 },
        Station { id: 142, x: 555, y: 405 },
        Station { id: 143, x: 610, y: 410 },
        Station { id: 144, x: 40, y: 400 },
        Station { id: 145, x: 75, y: 397 },
        Station { id: 146, x: 107, y: 392 },
        Station { id: 147, x: 140, y: 395 },
        Station { id: 148, x: 170, y: 412 },
        Station { id: 149, x: 200, y: 395 },

        Station { id: 150, x: 225, y: 375 },
        Station { id: 151, x: 250, y: 397 },
        Station { id: 152, x: 275, y: 375 },
        Station { id: 153, x: 295, y: 405 },
        Station { id: 154, x: 345, y: 390 },
        Station { id: 155, x: 367, y: 410 },
        Station { id: 156, x: 395, y: 420 },
        Station { id: 157, x: 450, y: 432 },
        Station { id: 158, x: 500, y: 420 },
        Station { id: 159, x: 510, y: 470 },

        Station { id: 160, x: 625, y: 445 },
        Station { id: 161, x: 670, y: 425 },
        Station { id: 162, x: 725, y: 420 },
        Station { id: 163, x: 110, y: 422 },
        Station { id: 164, x: 150, y: 435 },
        Station { id: 165, x: 210, y: 435 },
        Station { id: 166, x: 290, y: 435 },
        Station { id: 167, x: 330, y: 440 },
        Station { id: 168, x: 357, y: 460 },
        Station { id: 169, x: 390, y: 450 },

        Station { id: 170, x: 452, y: 470 },
        Station { id: 171, x: 650, y: 515 },
        Station { id: 172, x: 557, y: 467 },
        Station { id: 173, x: 637, y: 480 },
        Station { id: 174, x: 680, y: 455 },
        Station { id: 175, x: 710, y: 485 },
        Station { id: 176, x: 40, y: 457 },
        Station { id: 177, x: 75, y: 445 },
        Station { id: 178, x: 125, y: 462 },
        Station { id: 179, x: 180, y: 460 },

        Station { id: 180, x: 232, y: 482 },
        Station { id: 181, x: 275, y: 467 },
        Station { id: 182, x: 295, y: 490 },
        Station { id: 183, x: 315, y: 462 },
        Station { id: 184, x: 390, y: 480 },
        Station { id: 185, x: 445, y: 520 },
        Station { id: 186, x: 285, y: 502 },
        Station { id: 187, x: 542, y: 517 },
        Station { id: 188, x: 600, y: 500 },
        Station { id: 189, x: 75, y: 502 },

        Station { id: 190, x: 120, y: 525 },
        Station { id: 191, x: 145, y: 500 },
        Station { id: 192, x: 190, y: 547 },
        Station { id: 193, x: 260, y: 510 },
        Station { id: 194, x: 275, y: 535 },
        Station { id: 195, x: 315, y: 530 },
        Station { id: 196, x: 340, y: 487 },
        Station { id: 197, x: 352, y: 515 },
        Station { id: 198, x: 515, y: 545 },
        Station { id: 199, x: 600, y: 530 },

        // A secret node that cannot be reached.
        Station { id: 200, x: 600, y: 600 },
    ];

    return stations
        .into_iter()
        .map(|station| graph.add(station))
        .collect();
}

#[rustfmt::skip]
fn connect_taxi(graph: &mut Graph<Station, ConnectionType>, stations: &[NodeAddress]) {
    bidir(graph, &stations, 1, [8, 9], ConnectionType::Taxi);
    bidir(graph, &stations, 2, [10, 20], ConnectionType::Taxi);
    bidir(graph, &stations, 3, [4, 11, 12], ConnectionType::Taxi);
    bidir(graph, &stations, 4, [13], ConnectionType::Taxi);
    bidir(graph, &stations, 6, [7, 29], ConnectionType::Taxi);
    bidir(graph, &stations, 7, [17], ConnectionType::Taxi);
    bidir(graph, &stations, 8, [18, 19], ConnectionType::Taxi);
    bidir(graph, &stations, 9, [19, 20], ConnectionType::Taxi);
    bidir(graph, &stations, 10, [11, 21, 34], ConnectionType::Taxi);
    bidir(graph, &stations, 11, [22], ConnectionType::Taxi);
    bidir(graph, &stations, 12, [23], ConnectionType::Taxi);
    bidir(graph, &stations, 13, [14, 23, 24], ConnectionType::Taxi);
    bidir(graph, &stations, 14, [15, 25], ConnectionType::Taxi);
    bidir(graph, &stations, 15, [16, 26, 28], ConnectionType::Taxi);
    bidir(graph, &stations, 17, [29, 30, 42], ConnectionType::Taxi);
    bidir(graph, &stations, 18, [31, 43], ConnectionType::Taxi);
    bidir(graph, &stations, 19, [32], ConnectionType::Taxi);
    bidir(graph, &stations, 20, [33], ConnectionType::Taxi);
    bidir(graph, &stations, 21, [33], ConnectionType::Taxi);
    bidir(graph, &stations, 22, [23, 34, 35], ConnectionType::Taxi);
    bidir(graph, &stations, 23, [37], ConnectionType::Taxi);
    bidir(graph, &stations, 24, [37, 38], ConnectionType::Taxi);
    bidir(graph, &stations, 25, [38, 39], ConnectionType::Taxi);
    bidir(graph, &stations, 26, [27, 39], ConnectionType::Taxi);
    bidir(graph, &stations, 27, [28, 40], ConnectionType::Taxi);
    bidir(graph, &stations, 28, [41], ConnectionType::Taxi);
    bidir(graph, &stations, 29, [41, 42], ConnectionType::Taxi);
    bidir(graph, &stations, 30, [42], ConnectionType::Taxi);
    bidir(graph, &stations, 31, [43, 44], ConnectionType::Taxi);
    bidir(graph, &stations, 32, [33, 44, 45], ConnectionType::Taxi);
    bidir(graph, &stations, 33, [46], ConnectionType::Taxi);
    bidir(graph, &stations, 34, [47, 48], ConnectionType::Taxi);
    bidir(graph, &stations, 35, [36, 48, 65], ConnectionType::Taxi);
    bidir(graph, &stations, 36, [37, 49], ConnectionType::Taxi);
    bidir(graph, &stations, 37, [50], ConnectionType::Taxi);
    bidir(graph, &stations, 38, [50, 51], ConnectionType::Taxi);
    bidir(graph, &stations, 39, [51, 52], ConnectionType::Taxi);
    bidir(graph, &stations, 40, [41, 53], ConnectionType::Taxi);
    bidir(graph, &stations, 41, [54], ConnectionType::Taxi);
    bidir(graph, &stations, 42, [56, 72], ConnectionType::Taxi);
    bidir(graph, &stations, 43, [57], ConnectionType::Taxi);
    bidir(graph, &stations, 44, [58], ConnectionType::Taxi);
    bidir(graph, &stations, 45, [46, 58, 59, 60], ConnectionType::Taxi);
    bidir(graph, &stations, 46, [47, 61], ConnectionType::Taxi);
    bidir(graph, &stations, 47, [62], ConnectionType::Taxi);
    bidir(graph, &stations, 48, [62, 63], ConnectionType::Taxi);
    bidir(graph, &stations, 49, [50, 66], ConnectionType::Taxi);
    bidir(graph, &stations, 50, [67], ConnectionType::Taxi);
    bidir(graph, &stations, 51, [52, 67, 68], ConnectionType::Taxi);
    bidir(graph, &stations, 52, [69], ConnectionType::Taxi);
    bidir(graph, &stations, 53, [54, 69], ConnectionType::Taxi);
    bidir(graph, &stations, 54, [55, 70], ConnectionType::Taxi);
    bidir(graph, &stations, 55, [71], ConnectionType::Taxi);
    bidir(graph, &stations, 56, [91], ConnectionType::Taxi);
    bidir(graph, &stations, 57, [58, 73], ConnectionType::Taxi);
    bidir(graph, &stations, 58, [59, 74, 75], ConnectionType::Taxi);
    bidir(graph, &stations, 59, [75, 76], ConnectionType::Taxi);
    bidir(graph, &stations, 60, [61, 76], ConnectionType::Taxi);
    bidir(graph, &stations, 61, [62, 76, 78], ConnectionType::Taxi);
    bidir(graph, &stations, 62, [79], ConnectionType::Taxi);
    bidir(graph, &stations, 63, [64, 79, 80], ConnectionType::Taxi);
    bidir(graph, &stations, 64, [65, 81], ConnectionType::Taxi);
    bidir(graph, &stations, 65, [66, 82], ConnectionType::Taxi);
    bidir(graph, &stations, 66, [67, 82], ConnectionType::Taxi);
    bidir(graph, &stations, 67, [68, 84], ConnectionType::Taxi);
    bidir(graph, &stations, 68, [69, 85], ConnectionType::Taxi);
    bidir(graph, &stations, 69, [86], ConnectionType::Taxi);
    bidir(graph, &stations, 70, [71, 87], ConnectionType::Taxi);
    bidir(graph, &stations, 71, [72, 89], ConnectionType::Taxi);
    bidir(graph, &stations, 72, [90, 91], ConnectionType::Taxi);
    bidir(graph, &stations, 73, [74, 92], ConnectionType::Taxi);
    bidir(graph, &stations, 74, [92], ConnectionType::Taxi);
    bidir(graph, &stations, 75, [94], ConnectionType::Taxi);
    bidir(graph, &stations, 76, [77], ConnectionType::Taxi);
    bidir(graph, &stations, 77, [78, 95, 96], ConnectionType::Taxi);
    bidir(graph, &stations, 78, [79, 97], ConnectionType::Taxi);
    bidir(graph, &stations, 79, [98], ConnectionType::Taxi);
    bidir(graph, &stations, 80, [99, 100], ConnectionType::Taxi);
    bidir(graph, &stations, 81, [82, 100], ConnectionType::Taxi);
    bidir(graph, &stations, 82, [101], ConnectionType::Taxi);
    bidir(graph, &stations, 83, [101, 102], ConnectionType::Taxi);
    bidir(graph, &stations, 84, [85], ConnectionType::Taxi);
    bidir(graph, &stations, 85, [103], ConnectionType::Taxi);
    bidir(graph, &stations, 86, [103, 104], ConnectionType::Taxi);
    bidir(graph, &stations, 87, [88], ConnectionType::Taxi);
    bidir(graph, &stations, 88, [117], ConnectionType::Taxi);
    bidir(graph, &stations, 89, [105], ConnectionType::Taxi);
    bidir(graph, &stations, 90, [91, 105], ConnectionType::Taxi);
    bidir(graph, &stations, 91, [105, 107], ConnectionType::Taxi);
    bidir(graph, &stations, 92, [93], ConnectionType::Taxi);
    bidir(graph, &stations, 93, [94], ConnectionType::Taxi);
    bidir(graph, &stations, 94, [95], ConnectionType::Taxi);
    bidir(graph, &stations, 95, [122], ConnectionType::Taxi);
    bidir(graph, &stations, 96, [97, 109], ConnectionType::Taxi);
    bidir(graph, &stations, 97, [98, 109], ConnectionType::Taxi);
    bidir(graph, &stations, 98, [99, 110], ConnectionType::Taxi);
    bidir(graph, &stations, 99, [110, 112], ConnectionType::Taxi);
    bidir(graph, &stations, 100, [101, 112, 113], ConnectionType::Taxi);
    bidir(graph, &stations, 101, [114], ConnectionType::Taxi);
    bidir(graph, &stations, 102, [103, 115], ConnectionType::Taxi);
    bidir(graph, &stations, 104, [116], ConnectionType::Taxi);
    bidir(graph, &stations, 105, [106, 108], ConnectionType::Taxi);
    bidir(graph, &stations, 106, [107], ConnectionType::Taxi);
    bidir(graph, &stations, 107, [119], ConnectionType::Taxi);
    bidir(graph, &stations, 108, [117, 119, 135], ConnectionType::Taxi);
    bidir(graph, &stations, 109, [110, 124], ConnectionType::Taxi);
    bidir(graph, &stations, 110, [111], ConnectionType::Taxi);
    bidir(graph, &stations, 111, [112, 124], ConnectionType::Taxi);
    bidir(graph, &stations, 112, [125], ConnectionType::Taxi);
    bidir(graph, &stations, 113, [114, 125], ConnectionType::Taxi);
    bidir(graph, &stations, 114, [115, 126, 131, 132], ConnectionType::Taxi);
    bidir(graph, &stations, 115, [126, 127], ConnectionType::Taxi);
    bidir(graph, &stations, 116, [117, 118, 127], ConnectionType::Taxi);
    bidir(graph, &stations, 117, [129], ConnectionType::Taxi);
    bidir(graph, &stations, 118, [129, 134, 142], ConnectionType::Taxi);
    bidir(graph, &stations, 119, [136], ConnectionType::Taxi);
    bidir(graph, &stations, 120, [121, 144], ConnectionType::Taxi);
    bidir(graph, &stations, 121, [122, 145], ConnectionType::Taxi);
    bidir(graph, &stations, 122, [123, 146], ConnectionType::Taxi);
    bidir(graph, &stations, 123, [124, 137, 148, 149], ConnectionType::Taxi);
    bidir(graph, &stations, 124, [130, 138], ConnectionType::Taxi);
    bidir(graph, &stations, 125, [131], ConnectionType::Taxi);
    bidir(graph, &stations, 126, [127, 140], ConnectionType::Taxi);
    bidir(graph, &stations, 127, [133, 134], ConnectionType::Taxi);
    bidir(graph, &stations, 128, [142, 143, 160, 172, 188],ConnectionType::Taxi);
    bidir(graph, &stations, 129, [135, 142, 143], ConnectionType::Taxi);
    bidir(graph, &stations, 130, [131, 139], ConnectionType::Taxi);
    bidir(graph, &stations, 132, [140], ConnectionType::Taxi);
    bidir(graph, &stations, 133, [140, 141], ConnectionType::Taxi);
    bidir(graph, &stations, 134, [141, 142], ConnectionType::Taxi);
    bidir(graph, &stations, 135, [136, 143, 161], ConnectionType::Taxi);
    bidir(graph, &stations, 136, [162], ConnectionType::Taxi);
    bidir(graph, &stations, 137, [147], ConnectionType::Taxi);
    bidir(graph, &stations, 138, [150, 152], ConnectionType::Taxi);
    bidir(graph, &stations, 139, [140, 153, 154], ConnectionType::Taxi);
    bidir(graph, &stations, 140, [154, 156], ConnectionType::Taxi);
    bidir(graph, &stations, 141, [142, 158], ConnectionType::Taxi);
    bidir(graph, &stations, 142, [143, 158], ConnectionType::Taxi);
    bidir(graph, &stations, 143, [160], ConnectionType::Taxi);
    bidir(graph, &stations, 144, [145, 177], ConnectionType::Taxi);
    bidir(graph, &stations, 145, [146], ConnectionType::Taxi);
    bidir(graph, &stations, 146, [163, 147], ConnectionType::Taxi);
    bidir(graph, &stations, 147, [164], ConnectionType::Taxi);
    bidir(graph, &stations, 148, [149, 164], ConnectionType::Taxi);
    bidir(graph, &stations, 149, [150, 165], ConnectionType::Taxi);
    bidir(graph, &stations, 150, [151], ConnectionType::Taxi);
    bidir(graph, &stations, 151, [152, 165, 166], ConnectionType::Taxi);
    bidir(graph, &stations, 152, [153], ConnectionType::Taxi);
    bidir(graph, &stations, 153, [154, 166, 167], ConnectionType::Taxi);
    bidir(graph, &stations, 154, [155], ConnectionType::Taxi);
    bidir(graph, &stations, 155, [156, 167, 168], ConnectionType::Taxi);
    bidir(graph, &stations, 156, [157, 169], ConnectionType::Taxi);
    bidir(graph, &stations, 157, [158, 170], ConnectionType::Taxi);
    bidir(graph, &stations, 158, [159], ConnectionType::Taxi);
    bidir(graph, &stations, 159, [170, 172, 186, 198], ConnectionType::Taxi);
    bidir(graph, &stations, 160, [161, 173], ConnectionType::Taxi);
    bidir(graph, &stations, 161, [174], ConnectionType::Taxi);
    bidir(graph, &stations, 162, [175], ConnectionType::Taxi);
    bidir(graph, &stations, 163, [177, 178], ConnectionType::Taxi);
    bidir(graph, &stations, 164, [178, 179], ConnectionType::Taxi);
    bidir(graph, &stations, 165, [179, 180], ConnectionType::Taxi);
    bidir(graph, &stations, 166, [181, 183], ConnectionType::Taxi);
    bidir(graph, &stations, 167, [168, 183], ConnectionType::Taxi);
    bidir(graph, &stations, 169, [184], ConnectionType::Taxi);
    bidir(graph, &stations, 170, [185], ConnectionType::Taxi);
    bidir(graph, &stations, 171, [173, 175, 199], ConnectionType::Taxi);
    bidir(graph, &stations, 172, [187], ConnectionType::Taxi);
    bidir(graph, &stations, 173, [174, 188], ConnectionType::Taxi);
    bidir(graph, &stations, 174, [175], ConnectionType::Taxi);
    bidir(graph, &stations, 176, [177, 189], ConnectionType::Taxi);
    bidir(graph, &stations, 178, [189, 191], ConnectionType::Taxi);
    bidir(graph, &stations, 179, [191], ConnectionType::Taxi);
    bidir(graph, &stations, 180, [181, 192, 193], ConnectionType::Taxi);
    bidir(graph, &stations, 181, [182, 193], ConnectionType::Taxi);
    bidir(graph, &stations, 182, [195], ConnectionType::Taxi);
    bidir(graph, &stations, 183, [196], ConnectionType::Taxi);
    bidir(graph, &stations, 184, [185, 197], ConnectionType::Taxi);
    bidir(graph, &stations, 185, [186, 198], ConnectionType::Taxi);
    bidir(graph, &stations, 186, [198], ConnectionType::Taxi);
    bidir(graph, &stations, 187, [188, 198], ConnectionType::Taxi);
    bidir(graph, &stations, 188, [199], ConnectionType::Taxi);
    bidir(graph, &stations, 189, [190], ConnectionType::Taxi);
    bidir(graph, &stations, 190, [191, 192], ConnectionType::Taxi);
    bidir(graph, &stations, 191, [192], ConnectionType::Taxi);
    bidir(graph, &stations, 192, [194], ConnectionType::Taxi);
    bidir(graph, &stations, 193, [194], ConnectionType::Taxi);
    bidir(graph, &stations, 194, [195], ConnectionType::Taxi);
    bidir(graph, &stations, 195, [197], ConnectionType::Taxi);
    bidir(graph, &stations, 196, [197], ConnectionType::Taxi);
    bidir(graph, &stations, 198, [199], ConnectionType::Taxi);
}

#[rustfmt::skip]
fn connect_bus(graph: &mut Graph<Station, ConnectionType>, stations: &[NodeAddress]) {
    bidir(graph, &stations, 1, [46, 58], ConnectionType::Bus);
    bidir(graph, &stations, 3, [22, 23], ConnectionType::Bus);
    bidir(graph, &stations, 7, [42], ConnectionType::Bus);
    bidir(graph, &stations, 13, [14, 23, 52], ConnectionType::Bus);
    bidir(graph, &stations, 14, [15], ConnectionType::Bus);
    bidir(graph, &stations, 15, [29, 41], ConnectionType::Bus);
    bidir(graph, &stations, 22, [23, 34, 56], ConnectionType::Bus);
    bidir(graph, &stations, 23, [67], ConnectionType::Bus);
    bidir(graph, &stations, 29, [41, 42, 55], ConnectionType::Bus);
    bidir(graph, &stations, 34, [46, 63], ConnectionType::Bus);
    bidir(graph, &stations, 41, [52, 87], ConnectionType::Bus);
    bidir(graph, &stations, 42, [72], ConnectionType::Bus);
    bidir(graph, &stations, 46, [58, 78], ConnectionType::Bus);
    bidir(graph, &stations, 52, [67, 86], ConnectionType::Bus);
    bidir(graph, &stations, 55, [89], ConnectionType::Bus);
    bidir(graph, &stations, 63, [65, 79, 100], ConnectionType::Bus);
    bidir(graph, &stations, 65, [67, 82], ConnectionType::Bus);
    bidir(graph, &stations, 67, [82, 102], ConnectionType::Bus);
    bidir(graph, &stations, 72, [105, 107], ConnectionType::Bus);
    bidir(graph, &stations, 77, [78, 124], ConnectionType::Bus);
    bidir(graph, &stations, 78, [79], ConnectionType::Bus);
    bidir(graph, &stations, 82, [100, 140], ConnectionType::Bus);
    bidir(graph, &stations, 86, [87, 102, 116], ConnectionType::Bus);
    bidir(graph, &stations, 87, [105], ConnectionType::Bus);
    bidir(graph, &stations, 89, [105], ConnectionType::Bus);
    bidir(graph, &stations, 100, [111], ConnectionType::Bus);
    bidir(graph, &stations, 102, [127], ConnectionType::Bus);
    bidir(graph, &stations, 105, [107, 108], ConnectionType::Bus);
    bidir(graph, &stations, 107, [161], ConnectionType::Bus);
    bidir(graph, &stations, 108, [116, 135], ConnectionType::Bus);
    bidir(graph, &stations, 116, [127, 142], ConnectionType::Bus);
    bidir(graph, &stations, 122, [123, 144], ConnectionType::Bus);
    bidir(graph, &stations, 123, [124, 165, 144], ConnectionType::Bus);
    bidir(graph, &stations, 124, [153], ConnectionType::Bus);
    bidir(graph, &stations, 127, [133], ConnectionType::Bus);
    bidir(graph, &stations, 128, [135, 142, 161, 187, 199], ConnectionType::Bus);
    bidir(graph, &stations, 133, [140], ConnectionType::Bus);
    bidir(graph, &stations, 142, [157], ConnectionType::Bus);
    bidir(graph, &stations, 144, [163], ConnectionType::Bus);
    bidir(graph, &stations, 153, [154, 180, 184], ConnectionType::Bus);
    bidir(graph, &stations, 154, [156], ConnectionType::Bus);
    bidir(graph, &stations, 156, [157, 184], ConnectionType::Bus);
    bidir(graph, &stations, 157, [185], ConnectionType::Bus);
    bidir(graph, &stations, 161, [199], ConnectionType::Bus);
    bidir(graph, &stations, 163, [176, 191], ConnectionType::Bus);
    bidir(graph, &stations, 165, [180, 191], ConnectionType::Bus);
    bidir(graph, &stations, 176, [190], ConnectionType::Bus);
    bidir(graph, &stations, 180, [184, 190], ConnectionType::Bus);
    bidir(graph, &stations, 184, [185], ConnectionType::Bus);
    bidir(graph, &stations, 185, [187], ConnectionType::Bus);
    bidir(graph, &stations, 190, [191], ConnectionType::Bus);
}

#[rustfmt::skip]
fn connect_subways(graph: &mut Graph<Station, ConnectionType>, stations: &[NodeAddress]) {
    bidir(graph, &stations, 1, [46], ConnectionType::Underground);
    bidir(graph, &stations, 13, [46, 67, 89], ConnectionType::Underground);
    bidir(graph, &stations, 46, [74, 79], ConnectionType::Underground);
    bidir(graph, &stations, 67, [79, 89, 111], ConnectionType::Underground);
    bidir(graph, &stations, 79, [93, 111], ConnectionType::Underground);
    bidir(graph, &stations, 89, [128, 140], ConnectionType::Underground);
    bidir(graph, &stations, 111, [153, 163], ConnectionType::Underground);
    bidir(graph, &stations, 128, [140, 185], ConnectionType::Underground);
    bidir(graph, &stations, 140, [153], ConnectionType::Underground);
    bidir(graph, &stations, 153, [185], ConnectionType::Underground);
}

#[rustfmt::skip]
fn connect_ferries(graph: &mut Graph<Station, ConnectionType>, stations: &[NodeAddress]) {
    bidir(graph, &stations, 108, [115], ConnectionType::Ferry);
    bidir(graph, &stations, 115, [157], ConnectionType::Ferry);
    bidir(graph, &stations, 157, [194], ConnectionType::Ferry);
}

/// Bidirectionally link nodes.
fn bidir<L: IntoIterator<Item = usize>>(
    graph: &mut Graph<Station, ConnectionType>,
    stations: &[NodeAddress],
    from: usize,
    to: L,
    connection: ConnectionType,
) {
    let from = from - 1;
    // graph.link_to(&stations[from], &stations[to], connection);
    // graph.link_to(&stations[to], &stations[from], connection);
    for to in to {
        graph.link_bidir(&stations[to - 1], &stations[from], connection);
    }
}
