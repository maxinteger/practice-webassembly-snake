import init, {World, Direction, GameStatus} from 'snake_game'
import {rnd} from "./utils/rnd";


init().then((wasm) => {
    const FPS = 5
    const WORLD_WIDTH = 8
    const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH)

    const CELL_SIZE = 10;

    const world = World.new(WORLD_WIDTH, snakeSpawnIdx)
    const worldWidth = world.width()
    const points = document.getElementById('game-points')
    const statusLabel = document.getElementById('game-status')
    const gameControlBtn = document.getElementById('game-control-btn')
    const canvas = document.getElementById('snake-canvas') as HTMLCanvasElement;
    statusLabel.textContent = world.get_games_status_text()

    gameControlBtn.addEventListener('click', () => {
        world.start_game()
        gameControlBtn.remove()
        play()
        statusLabel.textContent = world.get_games_status_text()
    })

    canvas.width = worldWidth * CELL_SIZE
    canvas.height = worldWidth * CELL_SIZE

    const ctx = canvas.getContext('2d')

    document.addEventListener('keydown', (event) => {
        switch (event.code) {
            case 'ArrowUp':
                world.change_snake_dir(Direction.Up)
                break;
            case 'ArrowDown':
                world.change_snake_dir(Direction.Down)
                break;
            case 'ArrowLeft':
                world.change_snake_dir(Direction.Left)
                break;
            case 'ArrowRight':
                world.change_snake_dir(Direction.Right)
                break;
        }
    })

    function drawWorld() {
        ctx.beginPath()

        for (let x = 0; x < worldWidth + 1; x++) {
            ctx.moveTo(CELL_SIZE * x, 0)
            ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE)
        }
        for (let y = 0; y < worldWidth + 1; y++) {
            ctx.moveTo(0, CELL_SIZE * y)
            ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y)
        }

        ctx.stroke()
    }

    function drawSnake() {
        const cellsPtr = world.get_snake_cells()
        const length = world.get_snake_length()
        const snakeCells = new Uint32Array(wasm.memory.buffer, cellsPtr, length);

        snakeCells.slice().reverse().forEach((idx, i) => {
            const col = idx % worldWidth
            const row = Math.floor(idx / worldWidth)

            ctx.beginPath()
            ctx.fillStyle = i === 0 ? '#7878db' : '#000000'
            ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE)
            ctx.stroke()
        })
    }

    function drawReward() {
        const idx = world.get_reward_cell()
        const col = idx % worldWidth
        const row = Math.floor(idx / worldWidth)

        ctx.beginPath()
        ctx.fillStyle = "#ff0000";
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE)
        ctx.stroke()
    }

    function drawGameStatus() {
        statusLabel.textContent = world.get_games_status_text()
        points.textContent = world.get_points().toString()
    }

    function paint() {
        drawWorld()
        drawSnake()
        drawReward()

        drawGameStatus()

        if (world.get_games_status() === GameStatus.Won ) {
            alert('You Won!')
        }
    }

    function play() {
        const status = world.get_games_status()
        if (status === GameStatus.Lost || status === GameStatus.Won){
            return
        }

        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height)
            world.step()
            paint()
            requestAnimationFrame(play)
        }, 1000 / FPS)
    }

    paint()
})

