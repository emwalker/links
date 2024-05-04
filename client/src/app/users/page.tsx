'use client';

import { useEffect, useState } from "react";
import { fetchUsers, User } from '@/app/store';

export default function GET() {
  const [users, setUsers] = useState<User[]>([])

  useEffect(() => {
    (async function thunk() {
      const res = await fetchUsers()
      setUsers(res.items)
    })()
  }, [setUsers])

  return (
    <main className="p-24">
      <h1 className="font-bold text-xl mb-4">
        Users
      </h1>

      <ul className="mb-4">
        {
          users.map(({ id, name, username, is_admin }) => (
            <li key={id}>
              {
                name ? (
                  <span title={id}>{name} ({username}) {is_admin ? '[admin]' : ''}</span>
                ) : (
                  <span title={id}>{username} {is_admin ? '[admin]' : ''}</span>
                )
              }
            </li>
          ))
        }
      </ul>

      <div>
        <a href="/users/new" className="btn btn-blue">Add a user</a>
      </div>
    </main>
  )
}
