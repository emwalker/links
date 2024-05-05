'use client'

import { Card, Title, Button, Pagination } from '@mantine/core'
import { useEffect, useState } from 'react'
import { Page } from '@/components/Page'
import classes from './page.module.css'
import { User, fetchUsers } from '@/lib/store'

export default function GET() {
  const [activePage, setPage] = useState(1)
  const [perPage, setPerPage] = useState(10)
  const [users, setUsers] = useState<User[]>([])
  const [userCount, setTopicCount] = useState<number>(0)

  useEffect(() => {
    (async function thunk() {
      const { total, items, per_page } = await fetchUsers(activePage, perPage)
      setTopicCount(total)
      setPerPage(per_page)
      setUsers(items.slice(0, per_page))
    }())
  }, [setTopicCount, setPerPage, setUsers, activePage, perPage])

  const fractionalPageCount = userCount / perPage
  const integerPageCount = Math.floor(fractionalPageCount)
  const pageCount = fractionalPageCount > integerPageCount ? integerPageCount + 1 : integerPageCount

  return (
    <Page>
      <div className={classes.top}>
        <Title className={classes.title} order={2}>Users</Title>

        <Button
          color="green"
          variant="filled"
          component="a"
          href="/users/new"
          className={classes.addButton}
          >
            Add
        </Button>
      </div>

      <div className={classes.results}>
        {
          users.map(({ id, username }) => (
            <Card
              key={id}
              component="a"
              href={`/users/${id}`}
              padding="sm"
              radius="md"
              className={classes.card}>
              {username}
            </Card>
          ))
        }
      </div>

      <Pagination total={pageCount} value={activePage} onChange={setPage} my="sm" />
    </Page>
  )
}
