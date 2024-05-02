'use client';

import { useEffect, useState } from "react";

type Topic = {
  id: string,
  name: string,
}

type Response = {
  topic: Topic | null,
}

async function fetchTopic(topicId: string): Promise<Response> {
  const res = await fetch(`http://localhost:3000/api/topics/${topicId}`, { cache: 'no-cache' })

  if (!res.ok) {
    console.log('failed to fetch users: ', res)
  }

  return res.json()
}

const container = (name: string, inner: React.ReactNode): React.ReactNode => (
  <main className="p-24">
    <h1 className="font-bold text-xl mb-4">
      {name}
    </h1>

    {inner}
  </main>
)

export default function GET({ params }: { params: { id: string } }) {
  const [topic, setTopic] = useState<Topic | null>(null)
  const [requestInFlight, setRequestInFlight] = useState(true)
  const { id: topicId } = params

  useEffect(() => {
    async function thunk() {
      const res = await fetchTopic(topicId)
      setTopic(res.topic)
      setRequestInFlight(false)
      console.log('request completed.')
    }

    thunk()
  }, [setTopic, setRequestInFlight, topicId])

  if (topic == null) {
    if (requestInFlight) {
      return container('Fetching topic', <p>Waiting to hear back from server...</p>)
    } else {
      return container('Not found', <p>Topic {topicId} was not found.</p>)
    }
  }

  const { name } = topic

  return container(name,
    <p>
      Details about the topic include ...
    </p>
  )
}
