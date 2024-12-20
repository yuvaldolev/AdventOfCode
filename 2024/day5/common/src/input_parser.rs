use crate::{error, SafetyManualUpdates};

pub struct InputParser;

impl InputParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> error::Result<SafetyManualUpdates> {
        let (page_ordering_rules, pages_to_produce_per_update) =
            input
                .lines()
                .fold(Ok((Vec::new(), Vec::new())), |result, line| match result {
                    Ok((mut page_ordering_rules, mut pages_to_produce_per_update)) => {
                        if line.contains('|') {
                            let (before, after) = line.split_once('|').ok_or_else(|| {
                                error::Error::InvalidPageOrderingRuleFormat(line.to_owned())
                            })?;
                            page_ordering_rules.push((
                                before.parse().map_err(|e| {
                                    error::Error::ParsePageNumber(e, before.to_owned())
                                })?,
                                after.parse().map_err(|e| {
                                    error::Error::ParsePageNumber(e, after.to_owned())
                                })?,
                            ));
                        } else if line.contains(",") {
                            pages_to_produce_per_update.push(
                                line.split(',')
                                    .map(|number| {
                                        number.parse().map_err(|e| {
                                            error::Error::ParsePageNumber(e, number.to_owned())
                                        })
                                    })
                                    .collect::<Result<_, _>>()?,
                            )
                        }

                        Ok((page_ordering_rules, pages_to_produce_per_update))
                    }
                    Err(e) => Err(e),
                })?;

        Ok(SafetyManualUpdates::new(
            page_ordering_rules,
            pages_to_produce_per_update,
        ))
    }
}
