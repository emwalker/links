'use client'

import { useEffect, useState } from 'react'
import { Title } from '@mantine/core'
import { User, fetchUser } from '@/lib/store'
import { Page } from '@/components/Page'

const page = (name: string, inner: React.ReactNode): React.ReactNode => (
  <Page>
    <Title order={3}>
      {name}
    </Title>

    {inner}
  </Page>
)

export default function GET({ params }: { params: { id: string } }) {
  const [user, setUser] = useState<User | null>(null)
  const [requestInFlight, setRequestInFlight] = useState(true)
  const { id: userId } = params

  useEffect(() => {
    async function thunk() {
      const res = await fetchUser(userId)
      setUser(res.user)
      setRequestInFlight(false)
      console.log('request completed.')
    }

    thunk()
  }, [setUser, setRequestInFlight, userId])

  if (user == null) {
    if (requestInFlight) {
      return page('Fetching topic', <p>Waiting to hear back from server...</p>)
    }
      return page('Not found', <p>User {userId} was not found.</p>)
  }

  const { name, username } = user

  return page(name || username,
    <p>
      Details about the user include ...
    </p>
  )
}
