/// Union-Find (Disjoint Set Union) for tracking label equivalences.
pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            self.parent[root] = self.parent[self.parent[root]];
            root = self.parent[root];
        }
        root
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx != ry {
            self.parent[ry] = rx;
        }
    }
}

/// Input source for raster operations: either a file path or an in-memory Raster.
pub enum RasterInput {
    Path(String),
    Data(wbraster::Raster),
}

impl RasterInput {
    pub fn get_raster(self) -> Result<wbraster::Raster, String> {
        match self {
            RasterInput::Path(path) => wbraster::Raster::read(&path).map_err(|e| format!("Failed to read raster: {}", e)),
            RasterInput::Data(raster) => Ok(raster),
        }
    }
}

/// Input source for layer operations: either a file path or an in-memory Layer.
pub enum LayerInput {
    Path(String),
    Data(wbvector::Layer),
}

impl LayerInput {
    pub fn get_layer(self) -> Result<wbvector::Layer, String> {
        match self {
            LayerInput::Path(path) => wbvector::read(&path).map_err(|e| format!("Failed to read layer: {}", e)),
            LayerInput::Data(layer) => Ok(layer),
        }
    }
}
