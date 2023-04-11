import { useEffect, useState } from "react";
import { Button } from "./components/Button";
import { Input } from "./components/Input";

const Game = () => {
  const [username, setUsername] = useState<string | undefined>(undefined);
  const [startGame, setStartGame] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const run = async () => {
      if (username) {
        setLoading(true);
        const { main, setAuth } = await import("./game");
        setLoading(false);
        setAuth(username, "12345");
        main();
      }
    };

    if (startGame && username) {
      run();
    }
  }, [startGame, username]);

  const joinForm = !startGame ? (
    <div className="flex flex-col gap-3">
      <Input label="Username" type="text" value={username} setValue={setUsername} placeholder="jack" />
      <Button disabled={!username} text="Join Server" onClick={() => setStartGame(true)} />
    </div>
  ) : null;

  const statusText = loading ? <h1 className="text-4xl text-center my-4">Loading Game...</h1> : null;

  return (
    <>
      {joinForm}
      {statusText}
      <canvas className="bevy-instance__canvas" id="bevy" onContextMenu={(e) => e.preventDefault()}></canvas>
    </>
  );
};

export default Game;

