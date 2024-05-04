'use client'

import { Card, Title } from '@mantine/core'
import { useEffect, useState } from 'react'
import { Page } from '@/components/Page'
import { Topic, fetchTopics } from '@/app/store'
import classes from './page.module.css'

export default function GET() {
  const [topics, setTopics] = useState<Topic[]>([])

  useEffect(() => {
    (async function thunk() {
      const res = await fetchTopics()
      setTopics(res.items)
    }())
  }, [setTopics])

  return (
    <Page>
      <Title order={2}>Topics</Title>

      <div className={classes.results}>
        {
          topics.map(({ id, name }) => (
            <Card
              key={id}
              component="a"
              href={`/topics/${id}`}
              padding="sm"
              radius="md"
              className={classes.topicCard}>
              {name}
            </Card>
          ))
        }
      </div>
    </Page>
  )
}
