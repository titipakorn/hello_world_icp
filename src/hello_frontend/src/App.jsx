import { useState } from "react";
import { hello_backend } from "declarations/hello_backend";

function App() {
  const [greeting, setGreeting] = useState("");
  const [greetingList, setGreetingList] = useState([]);

  function handleSubmit(event) {
    event.preventDefault();
    const name = event.target.elements.name.value;
    if (name !== "") {
      hello_backend.greet(name).then((greeting_message) => {
        setGreeting(greeting_message);
      });
    }
    return false;
  }

  function getGreetingList() {
    hello_backend.submitted_names().then((profile) => {
      setGreetingList(profile);
    });
  }
  console.log(greetingList);
  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
        <button
          type="button"
          onClick={() => {
            getGreetingList();
          }}
        >
          ~Show Submitted Names~
        </button>
      </form>
      <section id="greeting">{greeting}</section>
      <div>
        <ul>
          {greetingList.map((greeting, index) => (
            <li key={`li-${index}`}>{greeting}</li>
          ))}
        </ul>
      </div>
    </main>
  );
}

export default App;
