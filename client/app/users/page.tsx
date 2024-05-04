'use client'

import { Card, Title } from '@mantine/core'
import { useEffect, useState } from 'react'
import { Page } from '@/components/Page'
import classes from './page.module.css'
import { User, fetchUsers } from '@/app/store'

export default function GET() {
  const [users, setUsers] = useState<User[]>([])

  useEffect(() => {
    (async function thunk() {
      const res = await fetchUsers()
      setUsers(res.items)
    }())
  }, [setUsers])

  return (
    <Page>
      <Title order={2}>Users</Title>

      <div className={classes.results}>
        {
          users.map(({ id, username }) => (
            <Card
              key={id}
              component="a"
              href={`/users/${id}`}
              padding="sm"
              radius="md"
              className={classes.userCard}>
              {username}
            </Card>
          ))
        }
      </div>
    </Page>
  )
}
