//
//  Shaders.metal
//  RustAnywhere
//

#include <metal_stdlib>
using namespace metal;

struct VertexIO {
    float4 position [[position]];
    float2 texCoord [[user(texturecoord)]];
};

static constexpr sampler s(filter::linear);

vertex VertexIO passthrough_vertex(
    constant float4 *position [[buffer(0)]],
    constant float2 *texCoord [[buffer(1)]],
    uint vertexId [[vertex_id]])
{
    VertexIO vert;
    vert.position = position[vertexId];
    vert.texCoord = texCoord[vertexId];
    return vert;
}

fragment half4 passthrough_fragment(
    VertexIO input [[stage_in]],
    texture2d<half> texture [[texture(0)]])
{
    return texture.sample(s, input.texCoord);
}
