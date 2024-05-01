export default function Home() {
  return (
    <main className="p-24">
      <h1 className="font-bold text-xl mb-4">
        Home
      </h1>

      <div>
        <ul>
          <li><a href="/users">Users</a></li>
          <li><a href="/topics">Topics</a></li>
        </ul>
      </div>
    </main>
  );
}
