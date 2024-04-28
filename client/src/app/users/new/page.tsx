'use client';

import { useRouter } from 'next/navigation'
import { FormEvent, useCallback } from 'react'

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
  const onSubmit = useCallback(async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const formData = new FormData(event.currentTarget)
    const username = formData.get('username');

    if (username != null) {
      await createUser(username as string)
    }

    router.push('/users')
    }, [router])
 
  return (
    <main className="p-24">
      <h1 className="font-bold text-xl mb-4">
        Add a user
      </h1>

      <form method="POST" action="/users" onSubmit={onSubmit}>
        <label htmlFor="username" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Username</label>
        <input id="username" name="username" type="text" placeholder="username"
          className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"></input>

        <input type="submit" value="Add" />
      </form>
    </main>
  )
}
