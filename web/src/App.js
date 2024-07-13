
import React, { useEffect, useState } from 'react';
import * as d3 from 'd3';

function App() {
  const [dependencies, setDependencies] = useState([]);
  
  useEffect(() => {
    fetch('/dependencies')
      .then(response => response.json())
      .then(data => setDependencies(data));
  }, []);
  
  useEffect(() => {
    if (dependencies.length > 0) {
    }
  }, [dependencies]);
  
  return (
    <div>
      <h1>Makefile Dependency Visualizer</h1>
      <div id="graph"></div>
    </div>
  );
}

export default App;
