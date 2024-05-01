'use client';

import { useRouter } from 'next/navigation'
import { ChangeEvent, useCallback, useState } from 'react'

async function createUser(username: string) {
  const body = JSON.stringify({ username })
  const res = await fetch('/api/users', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body
  })

  if (!res.ok) {
    throw new Error('Failed to create user')
  }

  const _ = await res.json()
}

export default function GET() {
  const router = useRouter()
  const [username, setUsername] = useState('')

  const updateUsername = useCallback(async (event: ChangeEvent<HTMLInputElement>) => {
    setUsername(event.target.value)
  }, [setUsername])

  const onClick = useCallback(async (_event: React.MouseEvent<HTMLElement>) => {
    if (username?.length > 0) {
      await createUser(username as string)
    }

    router.push('/users')
  }, [router, username])

  const addButtonEnabled = username?.length > 0

  return (
    <main className="p-24">
      <h1 className="font-bold text-xl mb-4">
        Add a user
      </h1>

      <form method="POST" action="/users">
        <div>
          <label htmlFor="username">Username</label>
          <input id="username" className="block test-base text-gray-900" name="username"
            type="text" placeholder="username" onChange={updateUsername} value={username} />
        </div>

        <button type="button" disabled={!addButtonEnabled}
          className="block text-base disabled:border-gray-500 rounded-sm text-sm border px-5 py-1 me-2 mt-2"
          onClick={onClick}>Add</button>
      </form>
    </main>
  )
}
