'use client'

import { Card, Title, Button, Pagination, Text, Group, Box } from '@mantine/core'
import { useEffect, useState } from 'react'
import { Page } from '@/components/Page'
import classes from './page.module.css'
import { User, fetchUsers } from '@/lib/store'
import useSession from '@/lib/useSession'

export default function GET() {
  const { session: { username } } = useSession()
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
        <Group className={classes.header}>
          <Title className={classes.title} order={2}>Users</Title>
          <Box>
            <Text>
              Your view of and preferences around other users within the current
              workspace.
            </Text>
          </Box>
        </Group>

        <Button
          component="a"
          href={`/${username}/users/new`}
          className={classes.addButton}
          >
            Add
        </Button>
      </div>

      <div className={classes.results}>
        {
          users.map(({ id, username: otherUsername }) => (
            <Card
              key={id}
              component="a"
              href={`/${username}/users/${id}`}
              padding="sm"
              radius="md"
              className={classes.card}>
              {otherUsername}
            </Card>
          ))
        }
      </div>

      <Pagination total={pageCount} value={activePage} onChange={setPage} my="sm" />
    </Page>
  )
}
