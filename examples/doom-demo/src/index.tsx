import {
  createSignal,
  createEffect,
  createElement,
  render,
  useAnimationFrame,
  useKeyboard,
  useMouse,
  createRef,
  onMount
} from '../dist/velocity-runtime.js';

// Game constants
const MAP_SIZE = 8;
const TILE_SIZE = 64;
const FOV = Math.PI / 3;
const MAX_DEPTH = 400;
const NUM_RAYS = 120;

// Map: 1 = wall, 0 = empty
const worldMap = [
  [1,1,1,1,1,1,1,1],
  [1,0,0,0,0,0,0,1],
  [1,0,1,0,0,1,0,1],
  [1,0,0,0,0,0,0,1],
  [1,0,1,1,1,0,0,1],
  [1,0,0,0,0,0,0,1],
  [1,0,0,0,0,1,0,1],
  [1,1,1,1,1,1,1,1]
];

function DoomGame() {
  const canvasRef = createRef();

  // Player state
  const [player, setPlayer] = createSignal({
    x: 150,
    y: 150,
    angle: 0,
    speed: 2,
    rotSpeed: 0.05
  });

  const [fps, setFps] = createSignal(0);
  const keys = useKeyboard();
  const mouse = useMouse();

  // Raycasting function
  function castRay(rayAngle) {
    let rayX = player().x;
    let rayY = player().y;

    const rayDirX = Math.cos(rayAngle);
    const rayDirY = Math.sin(rayAngle);

    let depth = 0;
    let hitWall = false;

    while (!hitWall && depth < MAX_DEPTH) {
      depth += 1;

      rayX = player().x + rayDirX * depth;
      rayY = player().y + rayDirY * depth;

      const mapX = Math.floor(rayX / TILE_SIZE);
      const mapY = Math.floor(rayY / TILE_SIZE);

      if (mapX < 0 || mapX >= MAP_SIZE || mapY < 0 || mapY >= MAP_SIZE) {
        hitWall = true;
        depth = MAX_DEPTH;
      } else if (worldMap[mapY][mapX] === 1) {
        hitWall = true;
      }
    }

    return depth;
  }

  // Render 3D view
  function render3D(ctx, width, height) {
    // Clear with floor/ceiling
    ctx.fillStyle = '#333';
    ctx.fillRect(0, 0, width, height / 2);
    ctx.fillStyle = '#666';
    ctx.fillRect(0, height / 2, width, height / 2);

    const p = player();
    const rayAngleStep = FOV / NUM_RAYS;

    for (let ray = 0; ray < NUM_RAYS; ray++) {
      const rayAngle = p.angle - FOV / 2 + ray * rayAngleStep;
      const distance = castRay(rayAngle);

      // Fix fisheye effect
      const correctedDistance = distance * Math.cos(rayAngle - p.angle);

      // Calculate wall height
      const wallHeight = (TILE_SIZE / correctedDistance) * 277;

      // Calculate brightness based on distance
      const brightness = Math.max(0, 255 - (correctedDistance / MAX_DEPTH) * 255);

      // Draw wall slice
      const x = (ray / NUM_RAYS) * width;
      const sliceWidth = width / NUM_RAYS;

      ctx.fillStyle = `rgb(${brightness}, ${brightness * 0.7}, ${brightness * 0.5})`;
      ctx.fillRect(
        x,
        (height - wallHeight) / 2,
        sliceWidth + 1,
        wallHeight
      );
    }
  }

  // Render 2D minimap
  function renderMinimap(ctx, mapSize) {
    const miniScale = mapSize / (MAP_SIZE * TILE_SIZE);

    // Draw map
    for (let y = 0; y < MAP_SIZE; y++) {
      for (let x = 0; x < MAP_SIZE; x++) {
        if (worldMap[y][x] === 1) {
          ctx.fillStyle = '#fff';
        } else {
          ctx.fillStyle = '#000';
        }
        ctx.fillRect(
          x * TILE_SIZE * miniScale,
          y * TILE_SIZE * miniScale,
          TILE_SIZE * miniScale,
          TILE_SIZE * miniScale
        );
      }
    }

    // Draw player
    const p = player();
    ctx.fillStyle = '#0f0';
    ctx.beginPath();
    ctx.arc(
      p.x * miniScale,
      p.y * miniScale,
      3,
      0,
      Math.PI * 2
    );
    ctx.fill();

    // Draw direction
    ctx.strokeStyle = '#0f0';
    ctx.beginPath();
    ctx.moveTo(p.x * miniScale, p.y * miniScale);
    ctx.lineTo(
      p.x * miniScale + Math.cos(p.angle) * 15,
      p.y * miniScale + Math.sin(p.angle) * 15
    );
    ctx.stroke();
  }

  // Game loop
  onMount(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    let frameCount = 0;
    let lastFpsUpdate = 0;

    useAnimationFrame((time, delta) => {
      const p = player();
      const k = keys();

      // Movement
      let newX = p.x;
      let newY = p.y;
      let newAngle = p.angle;

      // Keyboard controls
      if (k['ArrowUp'] || k['w'] || k['W']) {
        newX += Math.cos(p.angle) * p.speed;
        newY += Math.sin(p.angle) * p.speed;
      }
      if (k['ArrowDown'] || k['s'] || k['S']) {
        newX -= Math.cos(p.angle) * p.speed;
        newY -= Math.sin(p.angle) * p.speed;
      }
      if (k['ArrowLeft'] || k['a'] || k['A']) {
        newAngle -= p.rotSpeed;
      }
      if (k['ArrowRight'] || k['d'] || k['D']) {
        newAngle += p.rotSpeed;
      }

      // Mouse look (if mouse is moving)
      const m = mouse();
      if (m.deltaX !== 0) {
        newAngle += m.deltaX * 0.002;
      }

      // Collision detection
      const mapX = Math.floor(newX / TILE_SIZE);
      const mapY = Math.floor(newY / TILE_SIZE);

      if (mapX >= 0 && mapX < MAP_SIZE && mapY >= 0 && mapY < MAP_SIZE) {
        if (worldMap[mapY][mapX] === 0) {
          setPlayer({
            ...p,
            x: newX,
            y: newY,
            angle: newAngle
          });
        } else {
          // Only update angle if hit wall
          setPlayer({ ...p, angle: newAngle });
        }
      }

      // Render
      render3D(ctx, canvas.width, canvas.height);

      // Render minimap in corner
      ctx.save();
      ctx.translate(10, canvas.height - 160);
      renderMinimap(ctx, 150);
      ctx.restore();

      // FPS counter
      frameCount++;
      if (time - lastFpsUpdate > 1000) {
        setFps(frameCount);
        frameCount = 0;
        lastFpsUpdate = time;
      }
    });
  });

  return (
    <div class="game-container">
      <canvas
        ref={canvasRef}
        width="800"
        height="600"
        style={{
          border: '2px solid #333',
          display: 'block',
          margin: '0 auto',
          background: '#000'
        }}
      />
      <div class="controls">
        <h2>DOOM-Style Raycaster Demo</h2>
        <p>FPS: {fps}</p>
        <p>Controls:</p>
        <ul>
          <li>W/↑ - Move Forward</li>
          <li>S/↓ - Move Backward</li>
          <li>A/← - Turn Left</li>
          <li>D/→ - Turn Right</li>
          <li>Mouse - Look around</li>
        </ul>
        <p>Built with ⚡ Velocity Framework</p>
      </div>
    </div>
  );
}

// Mount the game
render(() => DoomGame(), document.getElementById('root'));
