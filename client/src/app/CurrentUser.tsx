'use client';

import { useEffect, useState } from 'react';
import { User, fetchUsers } from './store'

export default function CurrentUser() {
  const [users, setUsers] = useState<User[]>([])

  useEffect(() => {
    (async function thunk() {
      const res = await fetchUsers()
      setUsers(res.items)
    })()
  }, [setUsers])

  return (
    <select>
      {users.map(({ id, username }) => <option key={id}>{username}</option>)}
    </select>
  )
}
