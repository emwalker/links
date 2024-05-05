'use client'

import { useEffect, useState } from 'react'
import { Code, Group, Title, LoadingOverlay } from '@mantine/core'
import {
  IconUser,
  IconCircleLetterT,
  IconSwitchHorizontal,
  IconLogout,
  IconHome,
  IconSearch,
  IconBrandCodesandbox,
} from '@tabler/icons-react'
import { usePathname, useRouter } from 'next/navigation'
import Link from 'next/link'
import classes from './index.module.css'
import useSession from '@/lib/useSession'

const data = [
  { relativePathname: '/', label: 'Home', icon: IconHome },
  { relativePathname: '/users', label: 'Users', icon: IconUser },
  { relativePathname: '/topics', label: 'Topics', icon: IconCircleLetterT },
  { relativePathname: '/search', label: 'Search', icon: IconSearch },
]

type Props = {
  children: any
}

export function Page({ children }: Props) {
  const pathname = usePathname()
  const [active, setActive] = useState(pathname)
  const { session: { isLoggedIn, username }, isLoading, logout } = useSession()
  const router = useRouter()

  useEffect(() => {
    if (!isLoading && !isLoggedIn) {
      router.replace('/login')
    }
  }, [isLoading, isLoggedIn, router])

  if (isLoading) {
    return <LoadingOverlay />
  }

  const links = data.map((item) => {
    const fullPathname = `/${username}/${item.relativePathname}`
    return (
      <Link
        className={classes.link}
        data-active={fullPathname === active || undefined}
        href={fullPathname}
        key={item.label}
        onClick={() => {
        setActive(fullPathname)
      }}
    >
      <item.icon className={classes.linkIcon} stroke={1.5} />
      <span>{item.label}</span>
      </Link>
    )
  }
  )

  return (
    <div className={classes.container}>
      <nav className={classes.navbar}>
        <div className={classes.navbarMain}>
          <Group className={classes.header} justify="left">
            <IconBrandCodesandbox stroke={1.5} />
            <Title order={3}>Recommendations</Title>
            <Code>{username}</Code>
          </Group>
          {links}
        </div>

        <div className={classes.footer}>
          <Link href="/change-account" className={classes.link}>
            <IconSwitchHorizontal className={classes.linkIcon} stroke={1.5} />
            <span>Change account</span>
          </Link>

          <Link
            href="/logout"
            className={classes.link}
            onClick={(event) => {
              event.preventDefault()
              logout()
            }}
          >
            <IconLogout className={classes.linkIcon} stroke={1.5} />
            <span>Logout</span>
          </Link>
        </div>
      </nav>

      <div className={classes.content}>
        {children}
      </div>
    </div>
  )
}
