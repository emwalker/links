'use client';

import { useRouter } from 'next/navigation'
import { ChangeEvent, useCallback, useState } from 'react'

async function createUser(username: string) {
  const body = JSON.stringify({ username })
  const res = await fetch('/api/topics', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body
  })

  if (!res.ok) {
    throw new Error('Failed to create topic')
  }

  const _ = await res.json()
}

export default function GET() {
  const router = useRouter()
  const [name, setName] = useState('')

  const updateName = useCallback(async (event: ChangeEvent<HTMLInputElement>) => {
    setName(event.target.value)
  }, [setName])

  const onClick = useCallback(async (_event: React.MouseEvent<HTMLElement>) => {
    if (name?.length > 0) {
      await createUser(name as string)
    }

    router.push('/users')
  }, [router, name])

  const addButtonEnabled = name?.length > 0

  return (
    <main className="p-24">
      <h1 className="font-bold text-xl mb-4">
        Add a topic
      </h1>

      <form method="POST" action="/users">
        <div>
          <label htmlFor="name">Name</label>
          <input id="name" className="block test-base text-gray-900" name="name"
            type="text" placeholder="Name of topic" onChange={updateName} value={name} />
        </div>

        <button type="button" disabled={!addButtonEnabled}
          className="block text-base disabled:border-gray-500 rounded-sm text-sm border px-5 py-1 me-2 mt-2"
          onClick={onClick}>Add</button>
      </form>
    </main>
  )
}
