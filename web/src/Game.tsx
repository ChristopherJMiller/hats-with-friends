import { useEffect } from "react";

const Game = () => {
  useEffect(() => {
    import("./wasm/app").then(({ main }) => main());
  }, []);

  return (
    <canvas className="bevy-instance__canvas" id="bevy"></canvas>
  );
};

export default Game;

