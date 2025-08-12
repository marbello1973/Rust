import init, { add, rest, mul} from "../pkg/wasm_app.js";

async function runApp() {
  // Inicializar WASM
  await init();

  // Obtener elementos del DOM
  const number1Input = document.querySelector("#num1");
  const number2Input = document.querySelector("#num2");
  const calculateBtn = document.querySelector("#calculate");
  const resultDiv = document.querySelector("#result");
  const signo = document.querySelector("#signo");

  // Configurar el evento click
  calculateBtn.addEventListener("click", () => {
    // Validar y obtener los valores
    const a = Number(number1Input.value) || 0;
    const b = Number(number2Input.value) || 0;

    // Calcular usando WASM
    const suma = add(a, b);
    const restar = rest(a, b);
    const multi = mul(a, b);

    if (signo.value === "suma"){
      resultDiv.innerHTML = `<strong>Sumar:</strong> ${a} + ${b} = <span style="color: #00fd00;">${suma}</span>`;
    }else if (signo.value === "resta"){
        resultDiv.innerHTML = `<strong>Restar:</strong> ${a} - ${b} = <span style="color: #fd0000;">${restar}</span>`;
    }else if (signo.value === "mul"){
        resultDiv.innerHTML = ` <strong>Multiplicacion:</strong> ${a} * ${b} = <span style="color: #fd2399;">${multi}</span>`;
    }else{
    resultDiv.innerHTML = `<strong>Sumar:</strong> ${a} + ${b} = <span style="color: #00fd00;">${suma}</span>`;
    }
  });
  console.log("Aplicación WASM cargada correctamente");
}

// Iniciar la aplicación
runApp();
