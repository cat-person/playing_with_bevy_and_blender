

use std::io::Cursor;

use bevy::asset::AsyncReadExt;
use bevy::render::mesh::{VertexAttributeValues, Indices};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::utils::thiserror::{Error, self};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could load shader: {0}")]
    Io(#[from] std::io::Error),
}


#[derive(Default)]
pub struct StlAssetLoader;

impl AssetLoader for StlAssetLoader {
    type Asset = Mesh;
    type Settings = ();
    type Error = CustomAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let mut cursor = Cursor::new(&bytes);

            match stl_io::read_stl(&mut cursor) {
                Ok(indexed_mesh) => Ok(stl_to_triangle_mesh(&indexed_mesh)),
                Err(error) => Err(CustomAssetLoaderError::Io(error)),
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["stl"]
    }
}

pub struct StlLoaderPlugin;

impl Plugin for StlLoaderPlugin {
	fn build(&self, app: &mut App) {
		app.init_asset::<Mesh>()
           .init_asset_loader::<StlAssetLoader>();
	}
}

fn stl_to_triangle_mesh(stl: &stl_io::IndexedMesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let vertex_count = stl.faces.len() * 3;

    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut indices = Vec::with_capacity(vertex_count);

    for (i, face) in stl.faces.iter().enumerate() {
        for j in 0..3 {
            let vertex = stl.vertices[face.vertices[j]];
            positions.push([vertex[0], vertex[1], vertex[2]]);
            normals.push([face.normal[0], face.normal[1], face.normal[2]]);
            indices.push((i * 3 + j) as u32);
        }
    }

    let uvs = vec![[0.0, 0.0]; vertex_count];

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}