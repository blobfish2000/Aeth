use rand::Rng;
use std::convert::*;

const CHUNK_SIZE: f32 = 32.0;

#[derive(Debug)]
pub struct Streetmap {
    pub blocks: Vec<BlockData>,
    edges: Vec<EdgeData>,
    x_chunks: usize,
    y_chunks: usize,
    chunks: Vec<Chunk>,
}

pub type BlockIndex = usize;
pub type EdgeIndex = usize;
pub type ChunkIndex = usize;

#[derive(Debug)]
pub struct Chunk {
    center_x: f32,
    center_y: f32,
    chunk_x: usize,
    chunk_y: usize,
    blocks: Vec<BlockIndex>,
}

#[derive(Debug)]
pub struct BlockData {
    pub x: f32,
    pub y: f32,
    chunk_index: ChunkIndex,
    first_edge: Option<EdgeIndex>,
}

#[derive(Debug)]
pub struct EdgeData {
    target: BlockIndex,
    next_edge: Option<EdgeIndex>,
}

impl Streetmap {
    pub fn new(width: f32, height: f32) -> Streetmap {
        let mut chunks = Vec::new();
        let x_chunks = (width / CHUNK_SIZE).ceil() as usize;
        let y_chunks = (height / CHUNK_SIZE).ceil() as usize;

        for i in 0..(x_chunks * y_chunks) {
            let chunk_x = i as f32 % x_chunks as f32;
            let chunk_y = (i as f32 / x_chunks as f32).floor();
            chunks.push(Chunk {
                center_x: CHUNK_SIZE * 0.5 + chunk_x * CHUNK_SIZE,
                center_y: CHUNK_SIZE * 0.5 + chunk_y * CHUNK_SIZE,
                chunk_x: chunk_x as usize,
                chunk_y: chunk_y as usize,
                blocks: Vec::new(),
            });
        }

        Streetmap {
            blocks: Vec::new(),
            edges: Vec::new(),
            x_chunks,
            y_chunks,
            chunks,
        }
    }

    fn find_chunk(&mut self, x: f32, y: f32) -> ChunkIndex {
        let chunk_index_x = (x / CHUNK_SIZE).floor() as usize;
        let chunk_index_y = (y / CHUNK_SIZE).floor() as usize * self.y_chunks;
        chunk_index_x + chunk_index_y
    }

    fn add_block(&mut self, x: f32, y: f32) -> BlockIndex {
        let index = self.blocks.len();
        let chunk = self.find_chunk(x, y);
        self.blocks.push(BlockData {
            x: x,
            y: y,
            chunk_index: chunk,
            first_edge: None,
        });
        self.chunks[chunk].blocks.push(index);
        index
    }

    fn add_edge(&mut self, source: BlockIndex, target: BlockIndex) {
        let edge_index = self.edges.len();
        let block_data = &mut self.blocks[source];
        self.edges.push(EdgeData {
            target: target,
            next_edge: block_data.first_edge,
        });
        block_data.first_edge = Some(edge_index);
    }

    fn block_edges(&mut self, block: BlockIndex) -> Adjacent {
        Adjacent {
            streetmap: self,
            current_edge: self.blocks[block].first_edge,
        }
    }

    fn adjacent_chunks(&mut self, initial: ChunkIndex) -> Vec<ChunkIndex> {
        let hopefuls = [
            initial as isize - self.x_chunks as isize - 1,
            initial as isize - self.x_chunks as isize,
            initial as isize - self.x_chunks as isize + 1,
            initial as isize - 1,
            initial as isize,
            initial as isize + 1,
            initial as isize + self.x_chunks as isize - 1,
            initial as isize + self.x_chunks as isize,
            initial as isize + self.x_chunks as isize + 1,
        ];

        let mut outvector = Vec::new();

        for index in &hopefuls {
            if *index >= 0
                && *index < self.chunks.len().try_into().unwrap()
                && !outvector.contains(&(*index as ChunkIndex))
            {
                outvector.push(*index as ChunkIndex);
            }
        }

        outvector
    }

    fn sq_dist_from_block(&self, a: BlockIndex, x: f32, y: f32) -> f32 {
        f32::powi(self.blocks[a].x - x, 2) + f32::powi(self.blocks[a].y - y, 2)
    }

    fn sq_dist_from_edge(&self, x: f32, y: f32) -> f32 {
        let x_dist = f32::min(x, self.x_chunks as f32 * CHUNK_SIZE - x);
        let y_dist = f32::min(y, self.y_chunks as f32 * CHUNK_SIZE - y);
        f32::min(f32::powi(x_dist, 2), f32::powi(y_dist, 2))
    }

    fn add_best_candidate(&mut self, num: usize) {
        let mut rng = rand::thread_rng();
        let mut candidates = Vec::new();
        let mut best_index = 0;
        for i in 0..num {
            candidates.push((
                rng.gen::<f32>() * (self.x_chunks as f32 * CHUNK_SIZE),
                rng.gen::<f32>() * (self.y_chunks as f32 * CHUNK_SIZE),
            ))
        }

        let mut furthest_dist = 0.0;
        for (i, candidate) in candidates.iter().enumerate() {
            let chunk = self.find_chunk(candidate.0, candidate.1);
            let adj = self.adjacent_chunks(chunk);
            let mut dist = self.sq_dist_from_edge(candidate.0, candidate.1);
            for chunk in adj.iter() {
                let blocks = self.chunks[*chunk].blocks.iter();
                for b_index in blocks {
                    let block_dist = self.sq_dist_from_block(*b_index, candidate.0, candidate.1);
                    if block_dist < dist {
                        dist = block_dist;
                    }
                }
            }
            if dist > furthest_dist {
                furthest_dist = dist;
                best_index = i;
            }
        }
        println!(
            "x: {:?}, y: {:?}, dist: {:?}",
            candidates[best_index].0, candidates[best_index].1, furthest_dist
        );
        self.add_block(candidates[best_index].0, candidates[best_index].1);
    }

    pub fn populate_blue_noise(&mut self, block_count: usize) {
        for count in 0..block_count {
            self.add_best_candidate(count * 10 + 10)
        }
    }
}

pub struct Adjacent<'map> {
    streetmap: &'map Streetmap,
    current_edge: Option<EdgeIndex>,
}

impl<'graph> Iterator for Adjacent<'graph> {
    type Item = BlockIndex;

    fn next(&mut self) -> Option<BlockIndex> {
        match self.current_edge {
            None => None,
            Some(edge_num) => {
                let edge = &self.streetmap.edges[edge_num];
                self.current_edge = edge.next_edge;
                Some(edge.target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //setting
    #[test]
    fn streetmap_chunk_dim() {
        let mut basic_sm = Streetmap::new(60.0, 120.0);
        assert_eq!(basic_sm.x_chunks, 2);
        assert_eq!(basic_sm.y_chunks, 4);
        assert_eq!(basic_sm.chunks.len(), 8);
    }

    #[test]
    fn streetmap_chunk_locs() {
        let mut basic_sm = Streetmap::new(60.0, 120.0);
        assert_eq!(basic_sm.chunks[0].center_x, 16.0);
        assert_eq!(basic_sm.chunks[0].center_y, 16.0);
        assert_eq!(basic_sm.chunks[1].center_x, 48.0);
        assert_eq!(basic_sm.chunks[1].center_y, 16.0);
        assert_eq!(basic_sm.chunks[2].center_x, 16.0);
        assert_eq!(basic_sm.chunks[2].center_y, 48.0);
        assert_eq!(basic_sm.chunks[4].center_x, 16.0);
        assert_eq!(basic_sm.chunks[4].center_y, 80.0);
    }

    #[test]
    fn streetmap_with_blocks() {
        let mut basic_sm = Streetmap::new(60.0, 60.0);
        basic_sm.add_block(18.0, 20.0);
        basic_sm.add_block(40.0, 20.0);
        basic_sm.add_block(45.0, 40.0);
        basic_sm.add_block(30.0, 42.0);
        assert_eq!(basic_sm.blocks.len(), 4);
        assert_eq!(basic_sm.blocks[0].chunk_index, 0);
        assert_eq!(basic_sm.blocks[1].chunk_index, 1);
        assert_eq!(basic_sm.blocks[2].chunk_index, 3);
        assert_eq!(basic_sm.blocks[3].chunk_index, 2);
    }

    #[test]
    fn streetmap_with_edges() {
        let mut basic_sm = Streetmap::new(60.0, 60.0);
        basic_sm.add_block(18.0, 20.0);
        basic_sm.add_block(40.0, 20.0);
        basic_sm.add_block(45.0, 40.0);
        basic_sm.add_block(30.0, 42.0);
        basic_sm.add_edge(0, 1);

        assert_eq!(
            basic_sm.edges[basic_sm.blocks[0].first_edge.unwrap()].target,
            1
        );

        basic_sm.add_edge(0, 2);
        basic_sm.add_edge(0, 3);

        let mut count = 0;

        for edge in basic_sm.block_edges(0) {
            count += 1;
        }

        assert_eq!(count, 3);
    }

    #[test]
    fn blue_noise() {
        let mut sm = Streetmap::new(300.0, 200.0);
        sm.populate_blue_noise(30);
        assert_eq!(sm.blocks.len(), 30);
    }
}
