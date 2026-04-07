using './main.bicep'
extends '../shared/base.bicepparam'

param prefix = 'dev'
param config = {
  ...base.config
  tags: {
    ...base.config.tags
    team: 'platform'
  }
}

param locations = [
  ...base.locations
  'centralus'
]

param fullName = '${base.app.name}${prefix}'
