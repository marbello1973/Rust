import init, { MigrationSimulator } from '../pkg/app_wasm_pack.js';

class WorldMapSimulation {
    constructor() {
        this.canvas = document.getElementById('simulationCanvas');
        this.ctx = this.canvas.getContext('2d');
        this.mapSvg = document.getElementById('world-map');
        this.cityLabelsContainer = document.getElementById('city-labels-container');
        this.wasmInstance = null;
        this.simulator = null;
        this.isRunning = false;
        this.animationFrameId = null;
        this.majorCities = [];
        this.svgSize = { width: 2000, height: 1000 };
        this.scaleFactor = 1;

        // Bind de métodos
        this.togglePause = this.togglePause.bind(this);
        this.reset = this.reset.bind(this);
        this.resizeCanvas = this.resizeCanvas.bind(this);

        this.setupControls();
        this.initSimulation();
    }

    setupControls() {
        document.getElementById('pauseBtn').addEventListener('click', this.togglePause);
        document.getElementById('resetBtn').addEventListener('click', this.reset);
        window.addEventListener('resize', this.resizeCanvas);
    }

    async initSimulation() {
        try {
            this.wasmInstance = await init();
            this.setupWorldCities();
            this.createCityLabels();
            this.simulator = new MigrationSimulator(this.getCityCoordinates());
            document.getElementById('particleCount').textContent = this.simulator.particle_count();
            this.resizeCanvas();
            
            // Iniciar animación
            this.isRunning = true;
            this.animate();
        } catch (error) {
            console.error('Error al inicializar:', error);
            alert('Error al cargar la simulación. Por favor recarga la página.');
        }
    }

    setupWorldCities() {
        const cityCoordinates = {
            "Nueva York": [-74.00512, 40.71199],
            "Londres": [-0.1276, 51.5072],
            "Tokio": [139.6917, 35.6895],
            "Sídney": [151.2093, -33.8688],
            "Ciudad de México": [ -102.51162, 24.07321],
            "São Paulo": [-46.6333, -23.5505],
            "Shanghái": [121.4737, 31.2304],
            "Moscú": [37.6173, 55.7558],
            "El Cairo": [31.2357, 30.0444],
            "Nueva Delhi": [77.2090, 28.6139]
        };

        this.majorCities = Object.entries(cityCoordinates).map(([name, [lon, lat]]) => {
            // Ajuste para centrar mejor el mapa (reduce desplazamiento a la derecha)
            const x = ((lon + 180) * (this.svgSize.width / 360)) * 0.96 + 40;
            const y = (90 - lat) * (this.svgSize.height / 180);
            return { name, x, y };
        });
    }

    createCityLabels() {
        // Limpiar labels existentes
        this.cityLabelsContainer.innerHTML = '';
        
        // Crear labels para cada ciudad
        this.majorCities.forEach(city => {
            const label = document.createElement('div');
            label.className = 'city-label';
            label.textContent = city.name;
            label.style.left = `${city.x * this.scaleFactor}px`;
            label.style.top = `${city.y * this.scaleFactor}px`;
            this.cityLabelsContainer.appendChild(label);
        });
    }

    getCityCoordinates() {
        return this.majorCities.flatMap(city => [city.x, city.y]);
    }

    animate() {
        if (!this.isRunning) {
            if (this.animationFrameId) {
                cancelAnimationFrame(this.animationFrameId);
                this.animationFrameId = null;
            }
            return;
        }

        this.simulator.update();
        this.render();
        this.animationFrameId = requestAnimationFrame(() => this.animate());
    }

    render() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        
        const particles = new Float32Array(
            this.wasmInstance.memory.buffer,
            this.simulator.get_particles(),
            this.simulator.particle_count() * 2
        );
        
        this.drawCityMarkers();
        this.drawParticles(particles);
    }

    drawParticles(particles) {
        this.ctx.fillStyle = 'rgba(255, 87, 34, 0.8)';
        
        for (let i = 0; i < particles.length; i += 2) {
            const x = particles[i] * this.scaleFactor;
            const y = particles[i+1] * this.scaleFactor;
            
            this.ctx.beginPath();
            this.ctx.arc(x, y, 2 * this.scaleFactor, 0, Math.PI * 2);
            this.ctx.fill();
        }
    }

    drawCityMarkers() {
        this.ctx.fillStyle = '#2196F3';
        
        this.majorCities.forEach(city => {
            const x = city.x * this.scaleFactor;
            const y = city.y * this.scaleFactor;
            
            // Glow effect
            this.ctx.beginPath();
            this.ctx.arc(x, y, 10 * this.scaleFactor, 0, Math.PI * 2);
            this.ctx.fillStyle = 'rgba(33, 150, 243, 0.3)';
            this.ctx.fill();
            
            // Círculo de la ciudad
            this.ctx.beginPath();
            this.ctx.arc(x, y, 6 * this.scaleFactor, 0, Math.PI * 2);
            this.ctx.fillStyle = '#2196F3';
            this.ctx.fill();
        });
    }

    resizeCanvas() {
        const svgRect = this.mapSvg.getBoundingClientRect();
        this.canvas.width = svgRect.width;
        this.canvas.height = svgRect.height;
        
        // Calcular factor de escala
        this.scaleFactor = svgRect.width / this.svgSize.width;
        
        // Actualizar posición de labels
        this.updateCityLabelsPosition();
    }

    updateCityLabelsPosition() {
        const labels = document.querySelectorAll('.city-label');
        this.majorCities.forEach((city, index) => {
            if (labels[index]) {
                labels[index].style.left = `${city.x * this.scaleFactor}px`;
                labels[index].style.top = `${city.y * this.scaleFactor}px`;
                labels[index].style.fontSize = `${12 * this.scaleFactor}px`;
            }
        });
    }

    togglePause() {
        this.isRunning = !this.isRunning;
        const pauseBtn = document.getElementById('pauseBtn');
        pauseBtn.textContent = this.isRunning ? 'Pausar' : 'Reanudar';
        pauseBtn.style.backgroundColor = this.isRunning ? '#4CAF50' : '#FF5722';
        
        if (this.isRunning) {
            this.animate();
        }
    }

    reset() {
        // Cancelar animación actual
        if (this.animationFrameId) {
            cancelAnimationFrame(this.animationFrameId);
            this.animationFrameId = null;
        }
        
        // Reiniciar simulador
        this.simulator = new MigrationSimulator(this.getCityCoordinates());
        
        // Restablecer estado
        this.isRunning = true;
        document.getElementById('pauseBtn').textContent = 'Pausar';
        document.getElementById('pauseBtn').style.backgroundColor = '#4CAF50';
        
        // Comenzar nueva animación
        this.animate();
    }
}

// Inicialización cuando el DOM esté listo
document.addEventListener('DOMContentLoaded', () => {
    new WorldMapSimulation();
});