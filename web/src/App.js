import './App.css';
import { useState, useEffect } from 'react';

function App() {

  const[wasm, setWasm] = useState(null);

  useEffect(async () => {
    setWasm(await import("jack-vm"));
  }, []);

  return (
    <div className="App">
        {wasm ? wasm.greet() : null}
    </div>
  );
}

export default App;