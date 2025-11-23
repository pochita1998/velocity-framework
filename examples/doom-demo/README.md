# DOOM-Style Raycaster Demo

A classic DOOM-style 3D raycaster game built entirely with Velocity Framework to showcase its game development capabilities.

## Features

- **Real-time 3D Raycasting**: Classic DOOM-style pseudo-3D rendering
- **Smooth 60 FPS**: Using `useAnimationFrame` for game loop
- **Full Controls**: WASD/Arrow keys + mouse look
- **Minimap**: Real-time 2D overhead view
- **Collision Detection**: Can't walk through walls
- **Performance Monitoring**: Real-time FPS counter

## Technologies Demonstrated

### Velocity Framework Features Used:
- ✅ **Reactive Signals** - Player state management
- ✅ **Game Loop** - `useAnimationFrame` for 60 FPS rendering
- ✅ **Keyboard Input** - `useKeyboard()` for movement
- ✅ **Mouse Input** - `useMouse()` for camera control
- ✅ **Refs** - `createRef()` for Canvas access
- ✅ **Effects** - Reactive rendering updates

### Game Techniques:
- **Raycasting** - Classic pseudo-3D rendering (like Wolfenstein 3D/DOOM)
- **Fisheye Correction** - Proper perspective correction
- **Distance-based Shading** - Walls darker when farther away
- **Collision Detection** - Grid-based wall collision
- **Input Handling** - Multi-input support (keyboard + mouse)

## Run the Demo

```bash
cd examples/doom-demo
velocity dev --port 3000
```

Then open http://localhost:3000

## Controls

- **W / ↑** - Move Forward
- **S / ↓** - Move Backward
- **A / ←** - Turn Left
- **D / →** - Turn Right
- **Mouse** - Look around (move mouse left/right)

## How It Works

### Raycasting Algorithm

The demo uses a simplified DOOM-style raycasting algorithm:

1. **Cast rays** - For each vertical slice of the screen, cast a ray from player
2. **Wall detection** - March ray forward until hitting a wall
3. **Distance calculation** - Calculate distance to wall
4. **Height calculation** - Wall height inversely proportional to distance
5. **Rendering** - Draw vertical slice with appropriate height and shading

### Performance

- **Compile time**: ~5ms
- **Runtime**: 60 FPS constant
- **Bundle size**: ~33KB (gzipped)
- **HMR**: <50ms hot reload

## Code Structure

```
doom-demo/
├── src/
│   └── index.tsx       # Main game logic + raycasting
├── dist/
│   ├── index.js        # Compiled output
│   └── velocity-runtime.js
├── public/
│   └── style.css       # DOOM-themed styling
└── index.html          # Entry point
```

## Technical Details

### Map Format

The world map is a simple 2D array:
```javascript
const worldMap = [
  [1,1,1,1,1,1,1,1],  // 1 = wall
  [1,0,0,0,0,0,0,1],  // 0 = empty
  [1,0,1,0,0,1,0,1],
  // ... etc
];
```

### Player State

```typescript
{
  x: number,      // X position in pixels
  y: number,      // Y position in pixels
  angle: number,  // Viewing angle in radians
  speed: number,  // Movement speed
  rotSpeed: number // Rotation speed
}
```

### Ray Casting

For each screen column, we:
1. Calculate ray angle: `playerAngle - FOV/2 + (column * rayStep)`
2. March ray forward in small steps
3. Check if ray hit wall (map cell == 1)
4. Calculate wall height: `wallHeight = (TILE_SIZE / distance) * 277`
5. Apply fisheye correction: `distance * cos(rayAngle - playerAngle)`

## Expanding the Demo

This is a minimal raycaster. You could add:

- **Textures** - Wall textures using imageData
- **Sprites** - Enemies/items using sprite rendering
- **Weapons** - Shooting mechanics
- **Audio** - Sound effects using Web Audio API
- **Multiple Levels** - Level loading system
- **Enemies** - AI and pathfinding
- **Collectibles** - Health/ammo pickups

All of these are possible with Velocity Framework's game features!

## Why Velocity for Games?

- **Fast Compilation** - 5ms compile = instant iteration
- **Hot Module Replacement** - See changes in <50ms
- **Fine-grained Reactivity** - Update only what changed
- **Direct DOM Access** - Canvas refs for rendering
- **Small Bundle** - 33KB runtime, lightweight
- **Game Loop Built-in** - `useAnimationFrame` with auto-cleanup
- **Input System** - Keyboard/Mouse hooks ready to use

Built with ⚡ **Velocity Framework**
