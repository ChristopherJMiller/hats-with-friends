import Game from "./Game"

const App = () => {

  return (
    <div className="App flex flex-col gap-6 my-6 items-center">
      <h1 className="text-5xl font-bold">Hats with Friends</h1>
      <div>
        <h2 className="italic text-lg">This game is under development and automatically deployed from <a className="underline" target="__blank" href="https://github.com/ChristopherJMiller/hats-with-friends">here</a>.</h2>
        <p className="text-center">Powered by 🦀 and ❤️ using the <a className="underline" target="__blank" href="https://bevyengine.org/">Bevy Engine</a>.</p>
      </div>
      <div className="max-w-screen-xl">
        <Game />
      </div>
    </div>
  )
}

export default App
