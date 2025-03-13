// Winter Playful Menu Background Shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> time: f32;

@vertex
fn vertex(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Create a triangle that covers the whole screen
    let x = f32(vertex_index & 1u) * 2.0 - 1.0;
    let y = f32((vertex_index >> 1u) & 1u) * 2.0 - 1.0;
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>((x + 1.0) / 2.0, (y + 1.0) / 2.0);
    
    return out;
}

// Random function for procedural generation
fn random(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy, vec2<f32>(12.9898, 78.233))) * 43758.5453123);
}

// Snowflake shape
fn snowflake(uv: vec2<f32>, center: vec2<f32>, size: f32, seed: f32) -> vec3<f32> {
    let dist = distance(uv, center);
    
    // Basic circle
    var shape = smoothstep(size, size * 0.8, dist);
    
    // Add some variation based on angle for a more crystalline look
    let angle = atan2(uv.y - center.y, uv.x - center.x);
    let spikes = 6.0; // Hexagonal snowflake
    let spike_strength = 0.2;
    
    // Add spikes
    shape *= 1.0 + spike_strength * sin(angle * spikes + seed * 10.0);
    
    // Add inner detail
    shape *= 1.0 + 0.2 * sin(dist * 40.0 + time * 0.5);
    
    let color = random_color(seed);
    
    return vec3<f32>(shape) * color * smoothstep(size * 1.2, size * 0.8, dist);
}
 
 // Function to generate a random color
 fn random_color(seed: f32) -> vec3<f32> {
     let r = fract(sin(seed * 12.9898) * 43758.5453);
     let g = fract(sin((seed + 0.1) * 12.9898) * 43758.5453);
     let b = fract(sin((seed + 0.2) * 12.9898) * 43758.5453);
     return vec3<f32>(r, g, b);
 }

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    // Background gradient - cool blue winter colors
    let bg_top = vec3<f32>(0.5, 0.7, 0.9);    // Light blue
    let bg_bottom = vec3<f32>(0.2, 0.4, 0.8); // Deeper blue
    var color = mix(bg_bottom, bg_top, uv.y);
    
    // Add some subtle patterns to the background
    let pattern = sin(uv.x * 20.0) * sin(uv.y * 20.0) * 0.03;
    color += vec3<f32>(pattern, pattern, pattern * 1.5);
    
    // Generate snowflakes
    var snowflakes = 0.0;
    
    // Large snowflakes
    for (var i = 0.0; i < 10.0; i += 1.0) {
        let seed = i * 0.1;
        
        // Calculate position with some movement
        let speed = 0.1 + seed * 0.2;
        let x_pos = fract(seed + sin(time * 0.1 + seed * 10.0) * 0.1);
        let y_pos = fract(seed * 1.5 - time * speed * 0.1);
        
        let flake_pos = vec2<f32>(x_pos, y_pos);
        let flake_size = 0.02 + seed * 0.02;
        
        snowflakes += snowflake(uv, flake_pos, flake_size, seed).r;
    }
    
    // Medium snowflakes
    for (var i = 0.0; i < 20.0; i += 1.0) {
        let seed = i * 0.05 + 0.3;
        
        let speed = 0.15 + seed * 0.2;
        let x_pos = fract(seed + sin(time * 0.2 + seed * 5.0) * 0.05);
        let y_pos = fract(seed * 2.0 - time * speed * 0.15);
        
        let flake_pos = vec2<f32>(x_pos, y_pos);
        let flake_size = 0.01 + seed * 0.01;
        
        snowflakes += snowflake(uv, flake_pos, flake_size, seed).r * 0.7;
    }
    
    // Small background snowflakes/particles
    for (var i = 0.0; i < 30.0; i += 1.0) {
        let seed = i * 0.03 + 0.7;
        
        let speed = 0.2 + seed * 0.3;
        let x_pos = fract(seed * 3.0 + sin(time * 0.3 + seed * 2.0) * 0.03);
        let y_pos = fract(seed * 4.0 - time * speed * 0.2);
        
        let flake_pos = vec2<f32>(x_pos, y_pos);
        let flake_size = 0.005 + seed * 0.005;
        
        // Simpler particles for the small ones
        let dist = distance(uv, flake_pos);
        snowflakes += smoothstep(flake_size, flake_size * 0.5, dist) * 0.3;
    }
    
    // Add snowflakes to the scene
    color += vec3<f32>(snowflakes);
    
    // Add some twinkling stars/ice crystals
    for (var i = 0.0; i < 15.0; i += 1.0) {
        let seed = i * 0.07 + 0.5;
        let star_pos = vec2<f32>(
            random(vec2<f32>(seed, seed * 0.7)),
            random(vec2<f32>(seed * 1.5, seed * 0.3))
        );
        
        // Use a more interesting twinkle effect
        let twinkle = 0.5 + 0.5 * sin(time * (2.0 + seed) + seed * 10.0);
        let star_color = random_color(seed * 5.0); // Different seed for star color
        let star = smoothstep(0.005, 0.0, distance(uv, star_pos)) * twinkle;
        
        color += star_color * star;
    }
    
    // Add a subtle vignette effect
    let vignette = 1.0 - smoothstep(0.5, 1.5, length((uv - 0.5) * 2.0));
    color *= vignette * 1.1;
    
    // Add a bit of playful color variation
    let playful_color = sin(time * 0.2) * 0.05;
    color += vec3<f32>(playful_color, playful_color * 0.5, playful_color * 0.2);
    
    return vec4<f32>(color, 1.0);
}
