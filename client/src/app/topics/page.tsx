'use client';

import { useEffect, useState } from "react";

type Topic = {
  id: string,
  name: string,
}

type FetchTopicsResponse = {
  total: Number,
  items: Topic[],
  page: Number,
}

async function fetchTopics(): Promise<FetchTopicsResponse> {
  const res = await fetch('http://localhost:3000/api/topics', { cache: 'no-cache' })

  if (!res.ok) {
    throw new Error('Failed to fetch users')
  }

  return res.json()
}

export default function GET() {
  const [users, setUsers] = useState<Topic[]>([])

  useEffect(() => {
    async function thunk() {
      const res = await fetchTopics()
      setUsers(res.items)
    }

    thunk()
  }, [setUsers])

  return (
    <main className="p-24">
      <h1 className="font-bold text-xl mb-4">
        Topics
      </h1>

      <ul className="mb-4">
        {
          users.map(({ id, name }) => (
            <li key={id}>
              <span title={id}>{name}</span>
            </li>
          ))
        }
      </ul>

      <div>
        <a href="/topics/new" className="btn btn-blue">Add a topic</a>
      </div>
    </main>
  )
}
